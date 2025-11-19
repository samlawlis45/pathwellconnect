import streamlit as st
import pandas as pd
import graphviz
import plotly.express as px
from data_loader import load_data, get_timeline_events

# Page Config
st.set_page_config(page_title="Pathwell Connect Demo", layout="wide", page_icon="🚚")

# Load Data
data = load_data()

# Sidebar: Role Selection (Governance)
st.sidebar.title("Pathwell Connect")
st.sidebar.header("Governance & Visibility")
role = st.sidebar.selectbox(
    "Select User Role",
    ["Shipper (Admin)", "3PL / Broker", "Carrier"]
)

# Logic for Redaction based on Role
def redact_financials(amount_str):
    if role == "Carrier":
        return "██████"
    return amount_str

def can_view_upstream_risk():
    if role == "Carrier":
        return False
    return True

# --- APP LAYOUT ---

st.title("IntelligentLedger MVP Demo")
st.markdown(f"**Current View:** {role}")

# Tabs for the 3 Screens
tab1, tab2, tab3 = st.tabs(["1. Freight Ledger Timeline", "2. Attribution Graph", "3. Financial Impact"])

# --- SCREEN 1: TIMELINE ---
with tab1:
    st.header("Freight Ledger Timeline")
    st.markdown("Chronological sequence of normalized events.")
    
    timeline_df = get_timeline_events(data)
    
    # Interactive Timeline using DataFrame display (Simple MVP)
    # In a real app, we'd use a custom component or styled list
    
    for _, row in timeline_df.iterrows():
        # Visual Redaction logic for specific event types if needed
        # (e.g. Carrier might not see precise ERP codes, but for MVP we just show events)
        
        with st.container():
            col1, col2, col3 = st.columns([1, 2, 4])
            with col1:
                st.caption(row['event_ts'].strftime('%Y-%m-%d %H:%M'))
            with col2:
                st.markdown(f"**{row['event_type']}**")
                st.badge(row['category'])
            with col3:
                st.write(row['description'])
                st.caption(f"Source: {row['source_system']}")
            st.divider()

# --- SCREEN 2: ATTRIBUTION GRAPH ---
with tab2:
    st.header("Attribution Engine")
    st.markdown("Causal chain analysis: Root Cause → Outcome")
    
    # Build Graph using Graphviz
    graph = graphviz.Digraph()
    graph.attr(rankdir='LR')
    
    # Add Nodes & Edges from mock data
    # We filter edges for the demo flow: E2->E3->E4->VIS-103 etc
    edges = data['edges']
    
    # Helper to get label
    def get_label(node_id):
        # Try to find in events df
        row = timeline_df[timeline_df['event_id'] == node_id]
        if not row.empty:
            return row.iloc[0]['event_type']
        # Check charges
        if node_id.startswith("CHG"):
            return "CHARGE"
        if node_id.startswith("RISK"):
            return "WEATHER_RISK"
        return node_id

    # Filter interesting edges for clarity
    relevant_edges = edges[edges['edge_type'].isin(['CAUSES', 'RESULTS_IN'])]
    
    for _, edge in relevant_edges.iterrows():
        src = edge['from_node_id']
        dst = edge['to_node_id']
        
        # Governance: Hide upstream supplier/risk nodes from Carrier
        if not can_view_upstream_risk():
            if "RISK" in src or "TENDER_REJECT" in get_label(src):
                continue

        graph.node(src, label=get_label(src), shape='box', style='filled', fillcolor='#e8f0fe')
        graph.node(dst, label=get_label(dst), shape='box', style='filled', fillcolor='#e8f0fe')
        graph.edge(src, dst, label=edge['edge_type'])

    try:
        st.graphviz_chart(graph)
    except Exception:
        st.warning("⚠️ Visualization requires Graphviz ('dot' executable). Showing textual chain instead.")
        
        st.subheader("Causal Chain:")
        for _, edge in relevant_edges.iterrows():
            src = edge['from_node_id']
            dst = edge['to_node_id']
            if not can_view_upstream_risk():
                if "RISK" in src or "TENDER_REJECT" in get_label(src):
                    continue
            st.markdown(f"**{get_label(src)}** --[{edge['edge_type']}]--> **{get_label(dst)}**")
    
    if role == "Carrier":
        st.warning("Restricted View: Upstream root causes redacted by governance policy.")
    else:
        st.info("Root Cause Identified: Tender Rejection (TMS) + Weather Amplification.")

# --- SCREEN 3: FINANCIAL IMPACT ---
with tab3:
    st.header("Financial Impact Dashboard")
    
    if role == "Carrier":
        st.error("⛔ Access Denied: Financial data is restricted for Carrier role.")
    else:
        col1, col2 = st.columns(2)
        
        with col1:
            st.subheader("Cost Variance Analysis")
            # Mock aggregated financials
            impact_data = {
                "Category": ["Freight Base", "Detention", "SLA Penalty"],
                "Amount": [1200, 125, 450]
            }
            df_impact = pd.DataFrame(impact_data)
            
            fig = px.bar(df_impact, x="Category", y="Amount", title="Total Landed Cost Components", text_auto=True)
            st.plotly_chart(fig, width='stretch')
            
            st.metric("Total Margin Erosion", "-12%", delta_color="inverse")
            
        with col2:
            st.subheader("SKU-Level Impact")
            st.markdown("Unit economics affected by this disruption:")
            
            sku_data = pd.DataFrame({
                "SKU": ["SKU-A (Widgets)", "SKU-B (Gadgets)"],
                "Original Margin": ["28%", "35%"],
                "Actual Margin": ["24%", "23%"],
                "Impact": ["-4%", "-12%"]
            })
            st.table(sku_data)
            
            st.subheader("Remediation Actions")
            c1, c2 = st.columns(2)
            if c1.button("Dispute Detention Invoice"):
                st.toast("Dispute payload sent to ERP (SAP) ✅")
            
            if c2.button("Update TMS Lead Time"):
                st.toast("Routing guide updated in TMS ✅")

st.sidebar.markdown("---")
st.sidebar.caption("Pathwell Connect v0.1")
