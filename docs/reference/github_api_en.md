# GitHub API Reference for XiaoTian

This document provides an overview of the GitHub APIs that are relevant to the XiaoTian project, along with usage recommendations.

## Authentication

GitHub API requires authentication for most endpoints and to get higher rate limits:

- **Personal Access Token (PAT)**: Most straightforward authentication method
- **OAuth Apps**: For more complex authorization flows
- **GitHub Apps**: For integrations that act on behalf of users

For XiaoTian v0.2.0, we recommend using Personal Access Tokens with the following scopes:

- `repo` (for private repositories)
- `public_repo` (for public repositories only)

## Rate Limits

GitHub API enforces rate limits:

- Unauthenticated requests: 60 requests per hour
- Authenticated requests: 5,000 requests per hour

XiaoTian should implement caching and rate limit handling to avoid disruptions.

## REST API Endpoints

### Repository Information

```
GET /repos/{owner}/{repo}
```

**Purpose**: Retrieve basic repository information.

**Returns**:

- Repository metadata (name, description, URL)
- Star count
- Fork count
- Default branch
- Creation and update timestamps
- License information

**XiaoTian Usage**: Core API for repository subscription and metadata storage.

### Commits

```
GET /repos/{owner}/{repo}/commits
```

**Parameters**:

- `since`: ISO 8601 timestamp (e.g., `2023-01-01T00:00:00Z`)
- `until`: ISO 8601 timestamp
- `per_page`: Number of results per page (max 100)
- `page`: Page number for pagination

**Returns**:

- List of commits with SHA, message, author, and timestamp
- Modified files
- Stats (additions, deletions)

**XiaoTian Usage**: Track repository activity and generate update notifications.

### Issues

```
GET /repos/{owner}/{repo}/issues
```

**Parameters**:

- `state`: Filter by state (open, closed, all)
- `since`: ISO 8601 timestamp
- `per_page`: Number of results per page (max 100)
- `page`: Page number for pagination
- `labels`: Comma-separated list of label names

**Returns**:

- Issue title and body
- Status (open/closed)
- Labels and milestones
- Creator and assignees
- Comments count

**XiaoTian Usage**: Monitor issue activity and track project progress.

### Pull Requests

```
GET /repos/{owner}/{repo}/pulls
```

**Parameters**:

- `state`: Filter by state (open, closed, all)
- `sort`: Sort by (created, updated, popularity, long-running)
- `direction`: Sort direction (asc, desc)
- `per_page`: Number of results per page (max 100)
- `page`: Page number for pagination

**Returns**:

- PR title and body
- Branch information
- Review status
- Merge status
- Associated commits

**XiaoTian Usage**: Track code contributions and development activity.

### Releases

```
GET /repos/{owner}/{repo}/releases
```

**Parameters**:

- `per_page`: Number of results per page (max 100)
- `page`: Page number for pagination

**Returns**:

- Release title and description
- Version tag
- Publication date
- Assets (binaries, etc.)

**XiaoTian Usage**: Monitor new versions and changelog information.

## GraphQL API

GitHub's GraphQL API allows fetching multiple types of data in a single request.

**Endpoint**: `https://api.github.com/graphql`

**Example Query** (fetching repository data, recent commits, issues, and PRs):

```graphql
query {
  repository(owner: "owner", name: "repo") {
    name
    description
    stargazerCount
    forkCount

    # Recent commits
    defaultBranchRef {
      target {
        ... on Commit {
          history(first: 10) {
            nodes {
              messageHeadline
              committedDate
              author {
                name
              }
            }
          }
        }
      }
    }

    # Recent issues
    issues(last: 5, states: OPEN) {
      nodes {
        title
        createdAt
        author {
          login
        }
      }
    }

    # Recent PRs
    pullRequests(last: 5, states: OPEN) {
      nodes {
        title
        createdAt
        author {
          login
        }
      }
    }
  }
}
```

**XiaoTian Usage**: For more efficient data retrieval in future versions.

## Implementation with Octocrab

XiaoTian uses the `octocrab` Rust library for GitHub API interactions. Basic usage:

```rust
// Initialize client with personal access token
let octocrab = Octocrab::builder()
    .personal_token("YOUR_GITHUB_TOKEN".to_string())
    .build()?;

// Get repository information
let repo = octocrab
    .repos("owner", "repo")
    .get()
    .await?;

// Get recent commits
let commits = octocrab
    .repos("owner", "repo")
    .list_commits()
    .send()
    .await?;

// Get recent issues
let issues = octocrab
    .issues("owner", "repo")
    .list()
    .state(params::State::Open)
    .send()
    .await?;
```

## Best Practices

1. **Incremental Updates**: Use the `since` parameter to fetch only new data.
2. **Pagination**: Always handle API pagination for complete data retrieval.
3. **Caching**: Cache responses to reduce API calls.
4. **Error Handling**: Implement robust error handling, especially for rate limits.
5. **Conditional Requests**: Use etags and conditional requests to save on rate limits.

## Future Extensions

For later versions, XiaoTian could expand to use:

1. **Webhooks**: For real-time notifications of repository events.
2. **Repository Events API**: For more detailed activity tracking.
3. **GraphQL API**: For optimized data retrieval.
