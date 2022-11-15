// Get all persons
MATCH (n:Person) RETURN n

// Get person by username
MATCH (n:Person {username: 'roterkohl'}) Return n