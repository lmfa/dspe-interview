using System;
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
            "http://localhost:8080/candidates")
            ?? new();
    }
}
