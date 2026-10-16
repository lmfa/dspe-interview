-- Create interview_results table
CREATE TABLE interview_results (
	id INTEGER PRIMARY KEY,
	candidate_id INTEGER NOT NULL,
	technology_id INTEGER NOT NULL,
	seniority_level_id INTEGER NOT NULL,
	created_at DATETIME DEFAULT CURRENT_TIMESTAMP,

	FOREIGN KEY (candidate_id) REFERENCES candidates(id),
	FOREIGN KEY (technology_id) REFERENCES technologies(id),
	FOREIGN KEY (seniority_level_id) REFERENCES seniority_levels(id)
);

-- Create interview_question_results table to store responses and ratings
CREATE TABLE interview_question_results (
	id INTEGER PRIMARY KEY,
	interview_result_id INTEGER NOT NULL,
	question_id INTEGER NOT NULL,
	topic_id INTEGER NOT NULL,
	candidate_response TEXT NOT NULL,
	candidate_rating INTEGER NOT NULL,

	FOREIGN KEY (interview_result_id) REFERENCES interview_results(id),
	FOREIGN KEY (question_id) REFERENCES questions(id),
	FOREIGN KEY (topic_id) REFERENCES question_topics(id)
);
