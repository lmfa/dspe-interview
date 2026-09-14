
public class CreateQuestionDto
{
    public long SeniorityLevelId { get; set; }

    public long TopicId { get; set; }

    public string QuestionText { get; set; } = string.Empty;

    public string SuggestedAnswer { get; set; } = string.Empty;

    public List<long> TechnologyIds { get; set; } = [];
}
