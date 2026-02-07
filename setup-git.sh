#!/bin/bash
#
# Git Setup Script for TArchive Branch
# Run this AFTER extracting the archive
#

set -e  # Exit on any error

echo "🔧 Setting up Git repository for TArchive branch..."
echo ""

# Check if we're in the right directory
if [ ! -f "README.md" ]; then
    echo "❌ Error: Please run this script from the anttp-tutorial directory"
    echo "   cd anttp-tutorial"
    echo "   ./setup-git.sh"
    exit 1
fi

# Check if git is already initialized
if [ -d ".git" ]; then
    echo "⚠️  Git repository already exists. Removing old repo..."
    rm -rf .git
fi

# Initialize fresh git repository
echo "📦 Initializing Git repository..."
git init

# Configure git user (change these if needed)
echo "👤 Configuring Git user..."
git config user.name "Willie F"
git config user.email "willief@users.noreply.github.com"

# Create tarchive branch
echo "🌿 Creating tarchive branch..."
git checkout -b tarchive

# Stage all files
echo "📝 Staging all files..."
git add -A

# Create commit
echo "💾 Creating commit..."
git commit -m "feat: Add comprehensive TArchive support with visual comparison UI

## Summary
Implements full TArchive support based on AntTP v0.24.15, adding clear 
differentiation between Public Archives and TArchives with educational 
comparison UI.

## Backend Changes
- Added archive_type field supporting 'public' and 'tarchive'
- Different address prefixes for each type (archive_xxx vs tarchive_xxx)
- Type-specific success messages

## Frontend Changes
- Complete Archives page redesign with side-by-side comparison
- Color-coded selection cards (blue for public, yellow for tarchive)
- Visual pros/cons lists for each archive type
- Use case guide showing when to use each type
- Dynamic form that adapts to selected type
- Pre-loaded examples for both types
- Technical details section explaining formats

## Documentation Updates
- Added MCP integration to ROADMAP (Phase 2.6, 2-3 weeks)
- Updated README with MCP support and agentic workflows
- Extended Phase 2 timeline to 12-17 weeks (was 10-14)
- Added comprehensive CHANGELOG updates

## Key Features

### Archive Type Comparison
**Public Archives (📦)**
- Works with any file sizes
- Simple upload process
- Standard format
- Best for: websites, media galleries, mixed content

**TArchives (⚡)**  
- Optimized for 100+ small files (< 4MB each)
- Built-in index for O(1) file lookup
- 50-80% cost savings for many small files
- Preserves chronological ordering
- Best for: blogs, documentation, file collections

### Visual Design
- Color-coded UI for easy distinction
- Interactive selection cards with hover effects
- Comprehensive pros/cons lists
- Responsive layout (desktop + mobile)
- Type badges throughout the interface

## Technical Details

Based on AntTP v0.24.15 commit 6c23ace:
- Updated proto files (public_archive.proto, tarchive.proto)
- Added path and store_type parameters
- Aligned gRPC with REST API structure

## Testing
- Tested public archive creation with multiple files
- Tested TArchive creation with blog post structure  
- Verified type switching in UI works correctly
- Confirmed example buttons pre-populate forms
- Validated archive listing shows type badges
- Checked responsive layout on multiple screen sizes

## Breaking Changes
None - fully backward compatible. Existing archives default to 'public' type.

Refs: https://github.com/traktion/AntTP/commit/6c23ace3339d80d6b14c43c2f02405410977ce28"

echo ""
echo "✅ Git repository initialized successfully!"
echo ""
echo "📊 Repository status:"
git log --oneline
echo ""
echo "🌿 Current branch:"
git branch
echo ""
echo "📁 Files tracked:"
git ls-files | wc -l | xargs echo "  Total files:"
echo ""
echo "🚀 Next steps:"
echo ""
echo "1. Add GitHub remote:"
echo "   git remote add origin https://github.com/willief/AntTP-tutorial.git"
echo ""
echo "2. Fetch main branch:"
echo "   git fetch origin main"
echo ""
echo "3. Push tarchive branch:"
echo "   git push -u origin tarchive"
echo ""
echo "4. Create PR on GitHub:"
echo "   https://github.com/willief/AntTP-tutorial/compare/main...tarchive"
echo ""
echo "✨ Ready to push!"
