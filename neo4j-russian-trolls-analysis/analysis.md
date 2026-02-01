# Neo4j Russian Trolls Tweet Analysis

## Article Summary
NBC News published 200,000+ deleted Russian troll tweets from the Internet Research Agency (IRA) that targeted the 2016 US election. Neo4j graph database was used to analyze the coordinated propaganda network.

**Source:** https://neo4j.com/news/nbc-news-russian-trolls-tweets/

---

## Reflection Questions

### 1. Key Insights from Neo4j Analysis
**Graph databases revealed:**
- **Network coordination**: 200K tweets worked as organized networks, not isolated posts
- **Relationship patterns**: Exposed connections between fake accounts showing coordinated behavior
- **Propaganda spread**: Traced how inflammatory narratives propagated through the network
- **Account clustering**: Identified IRA-operated account groups working in concert

**Graph advantages over traditional databases:**
- Relationship-first architecture for complex connections
- Real-time traversal of "who retweeted who" in milliseconds
- Pattern detection for suspicious behaviors (coordinated posting, retweet cascades)
- Visual analytics to see network clusters

### 2. Why Deleted Tweets Matter
**Essential to analyze because:**
- Preserves evidence that would otherwise disappear
- Provides forensic record of 2016 disinformation tactics
- Creates pattern library to detect future campaigns
- Highlights ongoing threat - Russia continues targeting elections

**Risks highlighted:**
- Low-cost, low-risk information warfare with plausible deniability
- Without archiving, can't learn attacker tactics or develop countermeasures
- Sophistication - trolls masqueraded as legitimate activists (BLM, etc.)

### 3. Identifying Disinformation Patterns
**Neo4j detects:**
- **Coordination signatures**: Accounts tweeting within minutes of each other repeatedly
- **Bot networks**: Inhuman posting frequencies (100+ tweets/day), no organic conversations
- **Narrative tracking**: Map how false stories spread and mutate
- **Influence amplification**: Detect brigading and account clusters targeting specific posts

**Prevention:**
- Real-time alerts for suspicious coordination as it happens
- Compare new accounts against known troll behavioral baselines
- Early warning before campaigns reach critical mass

### 4. Neo4j Capabilities for 200K Tweets
- **Index-free adjacency**: Traversing relationships is O(1), analyzing connections is instant
- **Native graph storage**: Relationships as first-class citizens, no expensive JOINs
- **Cypher query language**: Declarative pattern matching optimized for graphs
- **Graph algorithms**: Community detection (Louvain), PageRank, shortest path, triangle counting
- **Scalability**: Handles millions of nodes and billions of relationships

### 5. Preventing Future Campaigns
**Detection strategies:**
- **Behavioral fingerprinting**: Profile troll posting patterns, language, engagement ratios
- **Anomaly detection**: Monitor for sudden tightly-connected account clusters
- **Temporal tracking**: Detect campaign phases from account creation → amplification
- **Centrality metrics**: Betweenness finds accounts bridging communities (sowing discord)
- **Predictive modeling**: Train ML on graph features to classify new accounts

---

## Neo4j Algorithms for Social Networks

| Algorithm | Use Case | Output |
|-----------|----------|--------|
| **Louvain** | Find coordinated troll clusters | Community IDs |
| **PageRank** | Identify influential/superspreader accounts | Importance scores |
| **Betweenness Centrality** | Find accounts bridging opposing groups | Centrality scores |
| **Triangle Counting** | Detect fake followers (low triangles = suspicious) | Triangle counts |
| **Node Similarity** | Find sock puppet accounts (same operator) | Similarity scores |
| **Shortest Path** | Trace how disinfo spreads from troll to mainstream | Path sequences |

---

## Example Cypher Queries

### Find Coordinated Retweets
```cypher
MATCH (a1:Account)-[:POSTED]->(t1:Tweet)-[:RETWEETS]->(original:Tweet)
MATCH (a2:Account)-[:POSTED]->(t2:Tweet)-[:RETWEETS]->(original)
WHERE a1 <> a2 
  AND abs(duration.between(t1.timestamp, t2.timestamp).seconds) < 300
WITH a1, a2, count(DISTINCT original) AS coordination_count
WHERE coordination_count > 10
RETURN a1.handle, a2.handle, coordination_count
ORDER BY coordination_count DESC
```

### Detect Bot Networks
```cypher
MATCH (a:Account)-[:POSTED]->(t:Tweet)
WHERE t.timestamp > datetime() - duration({days: 1})
WITH a, count(t) AS tweets_per_day
WHERE tweets_per_day > 100
RETURN a.handle, tweets_per_day
ORDER BY tweets_per_day DESC
```

### Community Detection
```cypher
CALL gds.louvain.stream('social-network')
YIELD nodeId, communityId
RETURN gds.util.asNode(nodeId).handle AS account, communityId
ORDER BY communityId
```

---

## Graph Data Model

### Nodes
- **Account**: `{id, handle, created_date, followers_count, is_suspected_troll, coordination_score}`
- **Tweet**: `{id, text, timestamp, retweet_count, toxicity_score}`
- **Hashtag**: `{tag, total_usage, suspected_campaign}`

### Relationships
- `(Account)-[:POSTED]->(Tweet)`
- `(Tweet)-[:RETWEETS]->(Tweet)`
- `(Tweet)-[:MENTIONS]->(Account)`
- `(Tweet)-[:USES_HASHTAG]->(Hashtag)`
- `(Account)-[:COORDINATES_WITH {coordination_score}]->(Account)`

---

## Twitter → Neo4j Pipeline Architecture

```
Twitter API → Kafka (Buffer) → Stream Processor (Rust) → Neo4j
                ↓                      ↓                    ↓
          Raw Storage          Enrichment           Graph Algorithms
                              (Sentiment,              (Detection)
                               Toxicity)                   ↓
                                                      Alert System
```

### Key Components
1. **Twitter Connector** (Rust): Stream tweets from API
2. **Kafka**: Buffer for reliable processing
3. **Stream Processor**: Enrich with sentiment/toxicity scores
4. **Neo4j Ingester**: Load tweets into graph
5. **Coordination Detector**: Run algorithms to find patterns

### Rust Example
```rust
// Neo4j ingestion
pub async fn ingest_tweet(&self, tweet: Value) -> Result<()> {
    let query = query(
        "CREATE (t:Tweet {id: $id, text: $text, timestamp: datetime($ts)})
         WITH t
         MATCH (a:Account {id: $author_id})
         CREATE (a)-[:POSTED]->(t)"
    )
    .param("id", tweet["id"].as_str().unwrap())
    .param("text", tweet["text"].as_str().unwrap())
    .param("ts", tweet["created_at"].as_str().unwrap())
    .param("author_id", tweet["author_id"].as_str().unwrap());
    
    self.graph.execute(query).await?;
    Ok(())
}
```

---

## Datasets for Practice

1. **Russian Troll Tweets** - 3M tweets from IRA (GitHub: fivethirtyeight/russian-troll-tweets)
2. **Twitter Election 2020** - 1.7M election-related tweets (Kaggle)
3. **COVID-19 Misinfo** - 1B+ COVID tweet IDs (GitHub: echen102/COVID-19-TweetIDs)
4. **SNAP Twitter Networks** - Stanford's 100K user social graph
5. **Reddit Comments** - Billions of threaded discussions (Pushshift)

### Sample Analysis Questions
- Which troll categories coordinated most frequently?
- How do bot networks coordinate hashtag campaigns?
- What are radicalization pathways in recommendation networks?
- How does misinformation spread from fringe to mainstream?

---

## Quick Start

```bash
# Setup Neo4j with Docker
docker run -p 7474:7474 -p 7687:7687 \
  -e NEO4J_AUTH=neo4j/password \
  neo4j:latest

# Load Russian troll CSV
LOAD CSV WITH HEADERS FROM 'file:///trolls.csv' AS row
MERGE (a:Account {author: row.author})
CREATE (t:Tweet {id: row.tweetid, text: row.content, 
                  timestamp: datetime(row.publish_date)})
CREATE (a)-[:POSTED]->(t)

# Run community detection
CALL gds.louvain.write('troll-network', {writeProperty: 'community'})

# Find suspicious coordination
MATCH (a1)-[:COORDINATES_WITH]->(a2)
WHERE a1.coordination_score > 0.8
RETURN a1.handle, a2.handle
```

---

## Resources
- Neo4j Graph Data Science: https://neo4j.com/docs/graph-data-science/
- GraphAcademy: https://graphacademy.neo4j.com/
- Dataset: https://github.com/fivethirtyeight/russian-troll-tweets
