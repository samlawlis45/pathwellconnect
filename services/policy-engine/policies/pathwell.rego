package pathwell.authz

import rego.v1

# Default deny - fail closed
default allow := false

# Allow if all conditions are met
allow if {
    # Agent must be valid and not revoked
    input.agent.valid == true
    input.agent.revoked == false
    
    # Request method must be in allowed methods
    input.request.method in allowed_methods
    
    # Request path must match allowed patterns
    path_matches_allowed_pattern
}

# Allowed HTTP methods
allowed_methods := {"GET", "POST", "PUT", "PATCH", "DELETE"}

# Check if path matches any allowed pattern
path_matches_allowed_pattern if {
    some pattern in default_allowed_patterns
    glob.match(pattern, ["/"], input.request.path)
}

# Default allowed patterns
default_allowed_patterns := ["**"]

# If data provides patterns, use those instead
path_matches_allowed_pattern if {
    data.pathwell.allowed_paths
    some pattern in data.pathwell.allowed_paths
    glob.match(pattern, ["/"], input.request.path)
}
