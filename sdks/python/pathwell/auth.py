"""Authentication and signing utilities for Pathwell SDK"""

import hashlib
import hmac
import base64
from pathlib import Path
from typing import Optional
from cryptography.hazmat.primitives import serialization
from cryptography.hazmat.primitives.asymmetric import rsa, ec
from cryptography.hazmat.backends import default_backend


def generate_key_pair() -> tuple[str, str]:
    """Generate a new RSA key pair for agent authentication.
    
    Returns:
        tuple: (private_key_pem, public_key_pem)
    """
    private_key = rsa.generate_private_key(
        public_exponent=65537,
        key_size=2048,
        backend=default_backend()
    )
    
    private_key_pem = private_key.private_bytes(
        encoding=serialization.Encoding.PEM,
        format=serialization.PrivateFormat.PKCS8,
        encryption_algorithm=serialization.NoEncryption()
    ).decode('utf-8')
    
    public_key_pem = private_key.public_key().public_bytes(
        encoding=serialization.Encoding.PEM,
        format=serialization.PublicFormat.SubjectPublicKeyInfo
    ).decode('utf-8')
    
    return private_key_pem, public_key_pem


def load_private_key(key_path: str) -> bytes:
    """Load a private key from a file path.
    
    Args:
        key_path: Path to the private key file
        
    Returns:
        bytes: Private key bytes
    """
    path = Path(key_path)
    if not path.exists():
        raise FileNotFoundError(f"Private key file not found: {key_path}")
    
    return path.read_bytes()


def sign_request(
    private_key_pem: bytes,
    method: str,
    path: str,
    body: Optional[bytes] = None,
    timestamp: Optional[str] = None,
) -> str:
    """Sign a request using the agent's private key.
    
    Args:
        private_key_pem: PEM-encoded private key
        method: HTTP method
        path: Request path
        body: Request body bytes (optional)
        timestamp: Request timestamp (optional)
        
    Returns:
        str: Base64-encoded signature
    """
    import time
    
    if timestamp is None:
        timestamp = str(int(time.time()))
    
    # Create signature payload
    body_hash = ""
    if body:
        body_hash = hashlib.sha256(body).hexdigest()
    
    payload = f"{method}\n{path}\n{timestamp}\n{body_hash}"
    
    # For MVP, we'll use HMAC with the private key
    # In production, this would use proper cryptographic signing
    private_key = serialization.load_pem_private_key(
        private_key_pem,
        password=None,
        backend=default_backend()
    )
    
    # Extract key material for HMAC (simplified for MVP)
    key_bytes = private_key.private_bytes(
        encoding=serialization.Encoding.DER,
        format=serialization.PrivateFormat.PKCS8,
        encryption_algorithm=serialization.NoEncryption()
    )
    
    # Use first 32 bytes as HMAC key
    hmac_key = key_bytes[:32]
    signature = hmac.new(hmac_key, payload.encode('utf-8'), hashlib.sha256).digest()
    
    return base64.b64encode(signature).decode('utf-8')

