
### ADD claude mcps
```
add-claude-mcps() {
  claude mcp add -- taskmaster-ai npx -y --package=task-master-ai task-master-ai
  
  claude mcp add context7 -- npx -y @upstash/context7-mcp

  claude mcp add sequential-thinking -- npx -y @modelcontextprotocol/server-sequential-thinking
  
  claude mcp add shadcn-ui -- npx shadcn@latest mcp 
```
### clear claude mcps
```
clear-claude-mcps() { 
  claude mcp list | grep -E "^[^:]+:" | awk -F: '{print $1}' | while read server; do
    claude mcp remove "$server"
  done
} 
```