use std::collections::HashMap;

type Date = String;
type ID = String;

struct Assignment {
    id: ID,
    name: String,
    description: String,
    date: Date,
    mark: Mark,
    feedback: String,
}

struct Marks {
    id: ID,
    student_id: ID,
    subject: Subject,
    assignments: Vec<Assignment>,
}

struct Student {
    id: ID,
    school_id: ID,
    name: String,
    age: u8,
    grade_level: GradeLevel,
    interests: Vec<InterestType>,
    personality_type: PersonalityType,
    learning_style: LearningStyle,
    skills: HashMap<String, SkillLevel>,
    goals: Vec<Goal>,
    projects: Vec<Project>,
    peer_matches: Vec<PeerMatch>,
    languages_spoken: Vec<Language>,
    nationality: Nationality,
}

struct School {
    id: ID,
    name: String,
    address: String,
    city: String,
    state: String,
    zip: String,
}

// New structs
struct Goal {
    id: ID,
    student_id: ID,
    description: String,
    goal_type: GoalType,
    target_date: Date,
    status: GoalStatus,
}

struct Project {
    id: ID,
    student_id: ID,
    name: String,
    description: String,
    start_date: Date,
    end_date: Option<Date>,
    status: ProjectStatus,
    skills_used: Vec<String>,
}

struct PeerMatch {
    id: ID,
    student1_id: ID,
    student2_id: ID,
    match_date: Date,
    match_score: f32,
    match_reasons: Vec<MatchReason>,
    interaction_history: Vec<Interaction>,
}

struct Interaction {
    id: ID,
    peer_match_id: ID,
    date: Date,
    interaction_type: InteractionType,
    duration: std::time::Duration,
    feedback: Option<String>,
}

// Matching algorithm structure
struct MatchingAlgorithm {
    interest_weight: f32,
    skill_weight: f32,
    goal_weight: f32,
    personality_weight: f32,
    learning_style_weight: f32,
    past_success_weight: f32,
}
