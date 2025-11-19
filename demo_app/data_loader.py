import pandas as pd
import os

# Path to the dbt seeds (mock data)
DATA_PATH = os.path.join(os.path.dirname(os.path.dirname(__file__)), 'pathwell_connect_dbt', 'seeds')

def load_data():
    """
    Loads the CSV seeds into a dictionary of DataFrames.
    Simulates querying the Data Warehouse.
    """
    data = {}
    files = {
        'events': 'raw_tms_events.csv',
        'visibility': 'raw_visibility_events.csv',
        'risk': 'raw_risk_events.csv',
        'edges': 'raw_ledger_edges.csv',
        'charges': 'raw_audit_charges.csv',
        'shipments': 'raw_tms_shipments.csv',
        'orders': 'raw_erp_orders.csv'
    }
    
    for key, filename in files.items():
        try:
            df = pd.read_csv(os.path.join(DATA_PATH, filename))
            data[key] = df
        except FileNotFoundError:
            print(f"Warning: {filename} not found.")
            data[key] = pd.DataFrame()
            
    return data

def get_timeline_events(data, shipment_id='SHP-1001'):
    """
    Combines TMS, Visibility, and Risk events into a single timeline.
    """
    tms = data['events'][data['events']['shipment_id'] == shipment_id].copy()
    tms['category'] = 'TMS'
    
    vis = data['visibility'][data['visibility']['shipment_id'] == shipment_id].copy()
    vis['category'] = 'Visibility'
    
    # For demo, we assume risk is linked or we just grab the risk event
    risk = data['risk'].copy()
    risk['category'] = 'Risk'
    risk['event_type'] = risk['risk_type'] # Normalize column name
    
    # Combine
    combined = pd.concat([
        tms[['event_id', 'event_ts', 'event_type', 'description', 'category', 'source_system']],
        vis[['event_id', 'event_ts', 'event_type', 'description', 'category', 'source_system']],
        risk[['event_id', 'event_ts', 'event_type', 'description', 'category', 'source_system']]
    ])
    
    combined['event_ts'] = pd.to_datetime(combined['event_ts'])
    combined = combined.sort_values('event_ts')
    return combined

