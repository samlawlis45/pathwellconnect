# Pathwell Connect MVP

This repository contains the MVP for the Pathwell Connect IntelligentLedger.

## Components

1.  **Attribution Engine**: Python logic for causal analysis (`src/`).
2.  **Data Models**: dbt project defining the canonical schema (`pathwell_connect_dbt/`).
3.  **Demo App**: Streamlit interface (`demo_app/`).

## How to Run Locally

```bash
pip install -r demo_app/requirements.txt
streamlit run demo_app/app.py
```

## Deployment

### Option 1: Streamlit Community Cloud (Recommended)

1.  Go to [share.streamlit.io](https://share.streamlit.io/).
2.  Connect your GitHub account.
3.  Select this repository (`pathwellconnect`).
4.  Set the **Main file path** to `demo_app/app.py`.
5.  Click **Deploy**.

### Option 2: Vercel (Not Recommended for Streamlit)

Vercel is designed for static sites and serverless functions. Streamlit requires a persistent server with WebSockets, which often times out or fails on Vercel.

If you must use Vercel, you are seeing a 404 because Vercel doesn't know how to build the Python app by default. You would need a `vercel.json` redirecting to a Python serverless function, but Streamlit's architecture fights against this.

**We strongly recommend Option 1 for this demo.**

