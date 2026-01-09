"""Pathwell Client for making authenticated requests through the proxy"""

import time
import hashlib
from typing import Optional, Dict, Any
import requests
from .auth import load_private_key, sign_request


class PathwellClient:
    """Client for making requests through Pathwell Connect proxy."""
    
    def __init__(
        self,
        agent_id: str,
        private_key_path: str,
        proxy_url: str = "http://localhost:8080",
        target_url: Optional[str] = None,
    ):
        """Initialize Pathwell client.
        
        Args:
            agent_id: Agent identifier
            private_key_path: Path to private key file
            proxy_url: Pathwell proxy URL
            target_url: Target backend URL (if different from proxy)
        """
        self.agent_id = agent_id
        self.private_key = load_private_key(private_key_path)
        self.proxy_url = proxy_url.rstrip('/')
        self.target_url = target_url or proxy_url
        self.session = requests.Session()
    
    def call(
        self,
        method: str,
        url: str,
        headers: Optional[Dict[str, str]] = None,
        body: Optional[Any] = None,
        **kwargs
    ) -> requests.Response:
        """Make an authenticated request through Pathwell proxy.
        
        Args:
            method: HTTP method (GET, POST, etc.)
            url: Target URL (will be forwarded by proxy)
            headers: Additional headers
            body: Request body (dict will be JSON-encoded)
            **kwargs: Additional arguments passed to requests
            
        Returns:
            requests.Response: Response object
        """
        # Prepare headers
        req_headers = headers.copy() if headers else {}
        
        # Add Pathwell headers
        req_headers['X-Pathwell-Agent-ID'] = self.agent_id
        
        # Prepare body
        body_bytes = None
        if body is not None:
            if isinstance(body, dict):
                import json
                body_bytes = json.dumps(body).encode('utf-8')
                req_headers.setdefault('Content-Type', 'application/json')
            elif isinstance(body, str):
                body_bytes = body.encode('utf-8')
            elif isinstance(body, bytes):
                body_bytes = body
            else:
                body_bytes = str(body).encode('utf-8')
        
        # Extract path from URL for signing
        from urllib.parse import urlparse
        parsed = urlparse(url)
        path = parsed.path
        if parsed.query:
            path += f"?{parsed.query}"
        
        # Sign request
        timestamp = str(int(time.time()))
        signature = sign_request(
            self.private_key,
            method,
            path,
            body_bytes,
            timestamp
        )
        req_headers['X-Pathwell-Signature'] = signature
        req_headers['X-Pathwell-Timestamp'] = timestamp
        
        # Make request through proxy
        # The proxy will forward to the target URL
        proxy_url = f"{self.proxy_url}{path}"
        if parsed.query:
            proxy_url += f"?{parsed.query}"
        
        response = self.session.request(
            method=method,
            url=proxy_url,
            headers=req_headers,
            data=body_bytes,
            **kwargs
        )
        
        return response
    
    def get(self, url: str, **kwargs) -> requests.Response:
        """Make a GET request."""
        return self.call('GET', url, **kwargs)
    
    def post(self, url: str, body: Optional[Any] = None, **kwargs) -> requests.Response:
        """Make a POST request."""
        return self.call('POST', url, body=body, **kwargs)
    
    def put(self, url: str, body: Optional[Any] = None, **kwargs) -> requests.Response:
        """Make a PUT request."""
        return self.call('PUT', url, body=body, **kwargs)
    
    def patch(self, url: str, body: Optional[Any] = None, **kwargs) -> requests.Response:
        """Make a PATCH request."""
        return self.call('PATCH', url, body=body, **kwargs)
    
    def delete(self, url: str, **kwargs) -> requests.Response:
        """Make a DELETE request."""
        return self.call('DELETE', url, **kwargs)

