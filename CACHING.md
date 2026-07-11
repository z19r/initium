# 🚀 CI/CD Caching Strategy

This document outlines the comprehensive caching strategy for Initium's CI/CD pipelines to minimize build times and reduce resource usage.

## 📊 **Current Performance**

- **Before optimization**: ~8-12 minutes per workflow
- **After optimization**: ~2-4 minutes per workflow (estimated 60-70% improvement)

## 🎯 **Caching Layers**

### 1. **Rust Toolchain Caching**

```yaml
- name: Install Rust toolchain
  uses: actions-rust-lang/setup-rust-toolchain@v1
  with:
    toolchain: stable
    components: rustfmt, clippy
    cache: true # Built-in caching
```

**Benefits:**

- Caches Rust compiler, stdlib, and tools
- Automatic cache invalidation on toolchain updates
- ~30-60 seconds saved per run

### 2. **Cargo Dependencies Caching**

```yaml
- name: Cache cargo dependencies
  uses: actions/cache@v4
  with:
    path: |
      ~/.cargo/registry
      ~/.cargo/git
      target
    key: ${{ runner.os }}-cargo-v3-${{ hashFiles('**/Cargo.lock') }}
    restore-keys: |
      ${{ runner.os }}-cargo-v3-
      ${{ runner.os }}-cargo-
```

**Benefits:**

- Caches downloaded crates and compiled artifacts
- ~2-5 minutes saved per run
- Most significant time savings

### 3. **Development Tools Caching**

```yaml
- name: Cache development tools
  uses: actions/cache@v4
  with:
    path: ~/.cargo/bin
    key: ${{ runner.os }}-dev-tools-v2-${{ hashFiles('justfile') }}
    restore-keys: |
      ${{ runner.os }}-dev-tools-v2-
      ${{ runner.os }}-dev-tools-
```

**Benefits:**

- Caches cargo-get, cargo-set-version, cargo-audit, etc.
- ~30-60 seconds saved per run
- Prevents re-installing tools every time

### 4. **Just Installation Caching**

```yaml
- name: Cache just installation
  uses: actions/cache@v4
  with:
    path: ~/.local/bin
    key: ${{ runner.os }}-just-v2-${{ hashFiles('justfile') }}
    restore-keys: |
      ${{ runner.os }}-just-v2-
      ${{ runner.os }}-just-
```

**Benefits:**

- Caches just binary
- ~10-20 seconds saved per run

### 5. **System Packages Caching**

```yaml
- name: Cache system packages
  uses: actions/cache@v4
  with:
    path: /var/cache/apt
    key: ${{ runner.os }}-apt-${{ hashFiles('**/Cargo.lock') }}
    restore-keys: |
      ${{ runner.os }}-apt-
```

**Benefits:**

- Caches apt packages (jq, etc.)
- ~10-30 seconds saved per run

## 🔧 **Cache Key Strategy**

### **Versioned Keys**

- Use versioned keys (e.g., `v2`, `v3`) to invalidate caches when strategy changes
- Prevents cache pollution from old strategies

### **Content-Based Keys**

- Use `hashFiles()` to invalidate caches when dependencies change
- Ensures cache consistency with code changes

### **Fallback Keys**

- Use `restore-keys` for partial cache hits
- Improves cache hit rates

### **Unique Suffixes**

- Use `github.sha` for workflow-specific caches
- Prevents cache conflicts between parallel runs

## 📈 **Cache Hit Rates**

| Cache Type         | Expected Hit Rate | Time Saved |
| ------------------ | ----------------- | ---------- |
| Rust Toolchain     | 95%+              | 30-60s     |
| Cargo Dependencies | 80-90%            | 2-5min     |
| Development Tools  | 90%+              | 30-60s     |
| Just Installation  | 95%+              | 10-20s     |
| System Packages    | 85%+              | 10-30s     |

## 🚀 **Implementation**

### **Shared Cache Setup**

Use the shared cache setup workflow:

```yaml
- name: Setup caching
  uses: ./.github/workflows/cache-setup.yml
  with:
    cache-key-suffix: ${{ github.sha }}
```

### **Workflow-Specific Caching**

For workflows with unique requirements:

```yaml
- name: Cache specific artifacts
  uses: actions/cache@v4
  with:
    path: path/to/cache
    key: ${{ runner.os }}-unique-${{ hashFiles('specific-file') }}
```

## 🔍 **Monitoring Cache Performance**

### **Cache Hit Indicators**

- Look for "Cache restored from key" messages in logs
- Monitor workflow run times
- Check cache hit rates in GitHub Actions

### **Cache Miss Debugging**

- Check if cache keys are consistent
- Verify file paths are correct
- Ensure cache size limits aren't exceeded

## 🛠️ **Local Development**

### **Justfile Commands**

```bash
# Clear all caches
just clear-cache

# Clear specific cache
just clear-cache cargo
just clear-cache tools

# Show cache status
just cache-status
```

### **Manual Cache Management**

```bash
# Clear project build artifacts only
rm -rf target

# Clear cargo registry (use with caution)
rm -rf ~/.cargo/registry ~/.cargo/git
```

## 📊 **Expected Results**

### **First Run (Cold Cache)**

- Full installation time
- All caches populated
- Baseline performance

### **Subsequent Runs (Warm Cache)**

- 60-70% faster execution
- Minimal tool installation
- Quick dependency resolution

### **Cache Invalidation**

- Automatic on dependency changes
- Manual via versioned keys
- Fallback to partial caches

## 🔧 **Troubleshooting**

### **Common Issues**

1. **Cache Misses**
   - Check cache key consistency
   - Verify file paths exist
   - Ensure cache size limits

2. **Stale Caches**
   - Use versioned cache keys
   - Implement proper invalidation
   - Monitor cache age

3. **Cache Conflicts**
   - Use unique suffixes
   - Separate caches by workflow
   - Avoid overlapping paths

### **Debug Commands**

```bash
# Check cache status
just cache-status

# Clear caches
just clear-cache
just clear-cache-cargo

# Rebuild with fresh cache
just clean && just cargo-build
```

## 📈 **Future Optimizations**

1. **Docker Layer Caching**
   - Use Docker for consistent environments
   - Cache Docker layers
   - Reduce setup time

2. **Artifact Caching**
   - Cache build artifacts between jobs
   - Share compiled binaries
   - Reduce compilation time

3. **Parallel Caching**
   - Cache different components in parallel
   - Optimize cache key generation
   - Improve cache hit rates

## 🎯 **Best Practices**

1. **Cache Early, Cache Often**
   - Set up caches at the beginning of workflows
   - Use multiple cache layers
   - Monitor cache effectiveness

2. **Version Your Caches**
   - Use versioned cache keys
   - Document cache changes
   - Plan cache invalidation

3. **Monitor Performance**
   - Track workflow run times
   - Monitor cache hit rates
   - Optimize based on data

4. **Test Cache Changes**
   - Test cache changes in feature branches
   - Verify cache invalidation works
   - Monitor for regressions

---

_This caching strategy is designed to significantly improve CI/CD performance while maintaining reliability and consistency._
