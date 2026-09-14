
public class QuestionDto
{
    public long Id { get; set; }

    public long TopicId { get; set; }

    public string TopicName { get; set; } = string.Empty;

    public long SeniorityLevelId { get; set; }

    public string SeniorityLevelName { get; set; } = string.Empty;

    public string QuestionText { get; set; } = string.Empty;

    public string SuggestedAnswer { get; set; } = string.Empty;

    public List<string> Technologies { get; set; } = [];
}

