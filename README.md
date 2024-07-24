# Peer-to-Peer Student Matching System

1. Key Features (with some overlap):
   - `AcademicInterests` - Match based on similar academic interests, a Discord
     channel where everyone is a mathematics enthusiast, for example.
   - `Goals` - Match based on similar goals, e.g., students aiming for a career
     in software engineering.
   - Demographics: `Age` (`GradeLevel`), `School`, `Location`, `Nationality`,
     languages spoken - self-explanatory
2. Secondary Features
   - Subject Proficiency, denoted through `Marks` - Try to match students into
     pairs/triplets/quadruplets where there will be a mix of students at
     different levels. Tutoring solidifies the strong students while improving
     the weaker students
   - `Skills` - Match based on complementary skills - this is cool for group
     projects where you need a mix of strengths, similar to a startup team 🚀
   - `LearningStyle`, `PersonalityType` - This is more for the peer-to-peer
     non-project based learning, I like the example of notemakers, if they share
     notes its all nice but some people have a different learning approach,
     same applies to introvert vs extrovert, so that conversations don't get
     dominated by one of the sides etc
3. Validation and fine-tuning
   - `Projects` - Project outcomes verify the inputs of students around how
     they feel about their capabilities and what the actual outcomes have been,
     this helps with both impostor syndrome strong performers and weak boasty
     performers
   - Past Interaction Success - this can be measured by the total time spent in
     the community and level of engagement same as you perform product analytics,
     might take forms of normalized anonymous feedback forms too and it can help
     to train the model

Now I think that based on those features a succint problem can be defined where
a specific model that predicts the compatibility of a student-to-student can be
trained

We are left with an embedding space of points represented by students where we
can find the distance between each point

Something along the lines of

```rust
fn calculate_primary_match_score(s1: &Student, s2: &Student) -> f32 {
    let interest_dist = calculate_interest_similarity(
        &student1.academic_interests,
        &student2.academic_interests
    );
    let goal_dist = calculate_goal_alignment(
        &student1.goals,
        &student2.goals
    );
    let demographic_dist = calculate_demographic_compatibility(
        student1,
        student2
    );

    let weights_sum = INTEREST_WEIGHT + GOAL_WEIGHT + DEMOGRAPHIC_WEIGHT;
    let dist_sum = (
       interest_dist * INTEREST_WEIGHT +
       goal_dist * GOAL_WEIGHT +
       demographic_dist * DEMOGRAPHIC_WEIGHT
    );

    dist_sum / weights_sum
}
```

Now for productionizing Id probably lean towards a graph database like Neo4j,
something that supports vector search and relations without imposing a tabular
structure since startups need to move fast

Id try a SOTA approach of using a transformer model to embed the students and
then use a similarity search to find the closest students

It is worth to also run something like BigQuery AutoML to see if any of the
more traditional algorithms could also be good for matching, since it is easy
and quick to do and it might yield on-par or even better results

````mermaid
graph TD
    A[Student Data Input] --> B[Data Preprocessing]
    B --> C[Neo4j Graph Database]
    C --> D[Transformer Model]
    D --> E[Student Embedding]
    E --> |Update| C
    C --> F[Vector Similarity Search]
    C --> G[Graph Algorithms]
    C --> H[BigQuery AutoML]
    F --> I[Fast Approximate Matches]
    G --> J[Complex Relationship Matches]
    H --> K[Traditional ML Matches]
    I --> L[Ensemble Matching Algorithm]
    J --> L
    K --> L
    L --> M[Final Matches]
    M --> N[Match Evaluation]
    N --> O[Feedback Loop]
    O --> |Update Models| D
    O --> |Refine Algorithms| L
    P[Interaction Data] --> C
    P --> N```
````
