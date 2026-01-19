# Phase 1: Enhanced Policy with Trust Score Evaluation
# Package: pathwell.authz.v2
# Adds trust-aware policy evaluation for MTCA compliance

package pathwell.authz.v2

import rego.v1

# ========================================
# DEFAULT RULES
# ========================================

# Default deny - fail closed
default allow := false

# Default trust action - none if no trust context
default trust_action := "none"

# Default trust evaluation - passed if no trust context
default trust_evaluation_passed := true

# ========================================
# MAIN ALLOW RULE
# ========================================

# Allow if all conditions are met
allow if {
    # Agent must be valid and not revoked
    agent_is_valid

    # Trust score check must pass (or be skipped if no trust context)
    trust_check_passed

    # Request method must be in allowed methods
    method_is_allowed

    # Request path must match allowed patterns
    path_matches_allowed_pattern

    # Tenant governance allows (if applicable)
    tenant_policy_allows
}

# ========================================
# AGENT VALIDATION
# ========================================

agent_is_valid if {
    input.agent.valid == true
    input.agent.revoked == false
}

# ========================================
# TRUST SCORE EVALUATION (TRUST.ASSURE)
# ========================================

# Trust check passes if no trust context provided (backward compatible)
trust_check_passed if {
    not input.agent.trust_score
}

# Trust check passes if score is above threshold
trust_check_passed if {
    input.agent.trust_score
    input.agent.trust_score.composite_score >= trust_threshold
}

# Trust evaluation passed - detailed check
trust_evaluation_passed if {
    not input.agent.trust_score
}

trust_evaluation_passed if {
    input.agent.trust_score
    input.agent.trust_score.composite_score >= trust_threshold
}

# Determine trust action based on score
trust_action := "passed" if {
    input.agent.trust_score
    input.agent.trust_score.composite_score >= trust_threshold
}

trust_action := "warn" if {
    input.agent.trust_score
    input.agent.trust_score.composite_score < trust_threshold
    input.agent.trust_score.composite_score >= warn_threshold
}

trust_action := "block" if {
    input.agent.trust_score
    input.agent.trust_score.composite_score < warn_threshold
}

# Get trust thresholds from data or use defaults
trust_threshold := data.pathwell.trust_threshold if {
    data.pathwell.trust_threshold
}

trust_threshold := 0.3 if {
    not data.pathwell.trust_threshold
}

warn_threshold := data.pathwell.warn_threshold if {
    data.pathwell.warn_threshold
}

warn_threshold := 0.1 if {
    not data.pathwell.warn_threshold
}

# ========================================
# HTTP METHOD VALIDATION
# ========================================

method_is_allowed if {
    input.request.method in allowed_methods
}

# Allowed HTTP methods
allowed_methods := {"GET", "POST", "PUT", "PATCH", "DELETE"}

# ========================================
# PATH VALIDATION
# ========================================

# Check if path matches any allowed pattern
path_matches_allowed_pattern if {
    some pattern in effective_allowed_patterns
    glob.match(pattern, ["/"], input.request.path)
}

# Use data-provided patterns if available, otherwise default
effective_allowed_patterns := data.pathwell.allowed_paths if {
    data.pathwell.allowed_paths
}

effective_allowed_patterns := default_allowed_patterns if {
    not data.pathwell.allowed_paths
}

# Default allowed patterns (permissive)
default_allowed_patterns := ["**"]

# ========================================
# TENANT POLICY EVALUATION (TEN.GOV)
# ========================================

# No tenant context - allow by default
tenant_policy_allows if {
    not input.context.tenant_governance
}

# Tenant context with inherit scope - use default policies
tenant_policy_allows if {
    input.context.tenant_governance
    input.context.tenant_governance.policy_scope == "inherit"
}

# Tenant context with override scope - use tenant-specific policies
tenant_policy_allows if {
    input.context.tenant_governance
    input.context.tenant_governance.policy_scope == "override"
    tenant_custom_policy_allows
}

# Tenant context with merge scope - must pass both default and custom
tenant_policy_allows if {
    input.context.tenant_governance
    input.context.tenant_governance.policy_scope == "merge"
    tenant_custom_policy_allows
}

# Custom tenant policy evaluation
tenant_custom_policy_allows if {
    not input.context.tenant_governance.custom_policies
}

tenant_custom_policy_allows if {
    input.context.tenant_governance.custom_policies
    count(input.context.tenant_governance.custom_policies) == 0
}

# If custom policies are specified, check them
tenant_custom_policy_allows if {
    input.context.tenant_governance.custom_policies
    count(input.context.tenant_governance.custom_policies) > 0
    # For now, we allow if custom_policies is present
    # Future: evaluate each custom policy
    true
}

# Tenant trust threshold override
effective_trust_threshold := input.context.tenant_governance.trust_threshold_override if {
    input.context.tenant_governance
    input.context.tenant_governance.trust_threshold_override
}

effective_trust_threshold := trust_threshold if {
    not input.context.tenant_governance
}

effective_trust_threshold := trust_threshold if {
    input.context.tenant_governance
    not input.context.tenant_governance.trust_threshold_override
}

# ========================================
# OUTPUT RULES (for policy response enrichment)
# ========================================

# Computed trust score used in evaluation
computed_trust_score := input.agent.trust_score.composite_score if {
    input.agent.trust_score
}

computed_trust_score := null if {
    not input.agent.trust_score
}

# Threshold that was applied
applied_threshold := effective_trust_threshold

# Tenant policy that was applied
applied_tenant_policy := input.context.tenant_governance.policy_scope if {
    input.context.tenant_governance
}

applied_tenant_policy := "none" if {
    not input.context.tenant_governance
}

# Warnings collection
warnings[warning] if {
    trust_action == "warn"
    warning := {
        "code": "TRUST_BELOW_THRESHOLD",
        "message": sprintf("Trust score %.2f is below threshold %.2f but above warn threshold", [computed_trust_score, trust_threshold]),
        "severity": "warning"
    }
}

warnings[warning] if {
    input.agent.trust_score
    input.agent.trust_score.dimensions.behavior < 0.3
    warning := {
        "code": "LOW_BEHAVIOR_SCORE",
        "message": "Agent has low behavioral trust score",
        "severity": "info"
    }
}

warnings[warning] if {
    input.agent.trust_score
    input.agent.trust_score.dimensions.alignment < 0.3
    warning := {
        "code": "LOW_ALIGNMENT_SCORE",
        "message": "Agent has low alignment trust score",
        "severity": "info"
    }
}
