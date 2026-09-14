using System.Net.Http.Json;

public class CandidateService
{
    private readonly HttpClient _httpClient;

    public CandidateService(HttpClient httpClient)
    {
        _httpClient = httpClient;
    }

    public async Task<List<CandidateDto>> GetCandidatesAsync()
    {
        return await _httpClient.GetFromJsonAsync<List<CandidateDto>>(
            "candidates")
            ?? new();
    }

    public async Task CreateCandidateAsync(string name)
    {
        var dto = new CreateCandidateDto
        {
            Name = name
        };

        var response = await _httpClient.PostAsJsonAsync(
            "candidates",
            dto);

        response.EnsureSuccessStatusCode();
    }
}
