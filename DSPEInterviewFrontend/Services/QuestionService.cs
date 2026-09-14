using System.Net.Http.Json;

public class QuestionService
{
    private readonly HttpClient _httpClient;

    public QuestionService(HttpClient httpClient)
    {
        _httpClient = httpClient;
    }

    public async Task<List<QuestionDto>> GetQuestionsAsync()
    {
        return await _httpClient.GetFromJsonAsync<List<QuestionDto>>(
            "questions")
            ?? new();
    }

    public async Task CreateQuestionAsync(CreateQuestionDto dto)
    {
        var response = await _httpClient.PostAsJsonAsync(
            "questions",
            dto);

        response.EnsureSuccessStatusCode();
    }
}
