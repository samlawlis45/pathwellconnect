FROM openpolicyagent/opa:latest

# Copy policy files
COPY policies/ /policies/

# Run OPA server with policies, binding to 0.0.0.0 to allow external access
CMD ["run", "--server", "--addr=0.0.0.0:8181", "--log-level=info", "/policies"]

