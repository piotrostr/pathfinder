// Existing enums
pub enum InterestType {
    Academic,
    Sports,
    Music,
    Art,
    // ...
}

pub enum Subject {
    Math,
    Biology,
    Chemistry,
    Physics,
    History,
    English,
    Geography,
    ComputerScience,
    // ...
}

pub enum Mark {
    A,
    B,
    C,
    D,
    F,
}

pub enum GradeLevel {
    PreSchool,
    FirstGrade,
    SecondGrade,
    ThirdGrade,
    FourthGrade,
    FifthGrade,
    SixthGrade,
    SeventhGrade,
    EighthGrade,
    NinthGrade,
    TenthGrade,
    EleventhGrade,
    TwelfthGrade,
}

pub enum PersonalityType {
    Introvert,
    Extrovert,
    Ambivert,
}

pub enum LearningStyle {
    Visual,
    Auditory,
    Kinesthetic,
    ReadWrite,
}

pub enum SkillLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

enum GoalType {
    Academic,
    Personal,
    Career,
}

enum GoalStatus {
    NotStarted,
    InProgress,
    Completed,
    Abandoned,
}

enum ProjectStatus {
    Planning,
    InProgress,
    Completed,
    OnHold,
}

enum MatchReason {
    SharedInterest(InterestType),
    ComplementarySkills(String, String),
    SimilarGoals,
    PersonalityMatch,
    LearningStyleMatch,
    SuccessfulPastInteraction,
}

enum InteractionType {
    VirtualMeeting,
    InPersonMeeting,
    CollaborativeProject,
    PeerTutoring,
    SocialEvent,
}

enum Language {
    English,
    Spanish,
    French,
    German,
    Chinese,
    Japanese,
    // ...
}

enum Nationality {
    Language(Language),
}
