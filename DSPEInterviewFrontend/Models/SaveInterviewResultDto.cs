public class SaveInterviewResultDto
{
    public long CandidateId { get; set; }
    public long TechnologyId { get; set; }
    public long SeniorityLevelId { get; set; }
    public List<InterviewQuestionResultDto> Questions { get; set; } = new();
}

public class InterviewQuestionResultDto
{
    public long QuestionId { get; set; }
    public long TopicId { get; set; }
    public string CandidateResponse { get; set; } = string.Empty;
    public int CandidateRating { get; set; }
}

public class InterviewResultResponseDto
{
    public long Id { get; set; }
    public long CandidateId { get; set; }
    public long TechnologyId { get; set; }
    public long SeniorityLevelId { get; set; }
    public string CreatedAt { get; set; } = string.Empty;
}
