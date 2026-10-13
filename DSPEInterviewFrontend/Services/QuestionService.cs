using System.Net.Http.Json;

public class QuestionService
{
    private readonly HttpClient _httpClient;

    public QuestionService(HttpClient httpClient)
    {
        _httpClient = httpClient;
    }

    public async Task<List<QuestionTopicDto>> GetTopicsAsync()
    {
        return await _httpClient.GetFromJsonAsync<List<QuestionTopicDto>>
            ("question-topics")
            ?? [];
    }

    public async Task<List<SeniorityLevelDto>> GetSeniorityLevelsAsync()
    {
        return await _httpClient.GetFromJsonAsync<List<SeniorityLevelDto>>
            ("seniority-levels")
            ?? [];
    }

    public async Task<List<TechnologyDto>> GetTechnologiesAsync()
    {
        return await _httpClient.GetFromJsonAsync<List<TechnologyDto>>
            ("technologies")
            ?? [];
    }

    public async Task<List<TechnologyDto>> GetTechnologiesByQuestionIdAsync(int questionId)
    {
        return await _httpClient.GetFromJsonAsync<List<TechnologyDto>>
            ($"questions/{questionId}/technologies")
            ?? [];
    }

    public async Task<List<QuestionDto>> GetQuestionsAsync()
    {
        var result = new List<QuestionDto>();
        result = await _httpClient.GetFromJsonAsync<List<QuestionDto>>
            ("questions")
            ?? [];

        var topics = await GetTopicsAsync();
        var seniorityLevels = await GetSeniorityLevelsAsync();
        var technologies = await GetTechnologiesAsync();

        await Task.WhenAll(result.Select(async q =>
        {
            q.TopicName = topics.FirstOrDefault(t => t.Id == q.TopicId)?.Name ?? "Unknown";
            q.SeniorityLevelName = seniorityLevels.FirstOrDefault(s => s.Id == q.SeniorityLevelId)?.Name ?? "Unknown";
            q.Technologies = [.. (await GetTechnologiesByQuestionIdAsync((int)q.Id)).Select(t => t.Name)];
        }));

        return result;
    }

    public async Task CreateQuestionAsync(CreateQuestionDto dto)
    {
        var response =
            await _httpClient.PostAsJsonAsync(
                "questions",
                dto);

        response.EnsureSuccessStatusCode();
    }
}
