-- Add migration script here
CREATE TABLE technologies
(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE
);

INSERT INTO technologies(name)
VALUES
('.NET'),
('Java'),
('Kotlin'),
('Python'),
('JavaScript'),
('TypeScript'),
('Rust');

CREATE TABLE question_technologies
(
    question_id INTEGER NOT NULL,
    technology_id INTEGER NOT NULL,

    PRIMARY KEY(question_id, technology_id),

    FOREIGN KEY(question_id)
        REFERENCES questions(id),

    FOREIGN KEY(technology_id)
        REFERENCES technologies(id)
);

CREATE TABLE seniority_levels
(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE
);

INSERT INTO seniority_levels(name)
VALUES
('Junior'),
('Medium'),
('Senior');

CREATE TABLE question_topics
(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    goal TEXT NOT NULL
);

INSERT INTO question_topics(name, goal)
VALUES
('.NET','Memory and Garbage collection, Value and ref types, Understanding of garbage collection, IDisposable, Assemblies, GAC, JIT, Reflection, AppDomains, CLR, Heaps'),
('C#','Indicates knowledge of C# language primitives like classes, interfaces, generics, events, dynamics, lamdba, attributes, exceptions, LINQ (queries, expressions, operations), anonymous methods/types'),
('Collections','Provide a goal for this topic'),
('Data persisting','Provide a goal for this topic'),
('.NET Multithreading','Thread/Task classes, Synchronization primitives, Async/await methods, TPL, PFX, PLINQ'),
('ASP .NET WebForms','Provide a goal for this topic'),
('ASP .Net MVC','Routes, Controllers, Standard conventions, Action filters, View results, View engines, Dependency resolvers'),
('Web Services/Web APIs','Provide a goal for this topic'),
('Windows Services','Provide a goal for this topic'),
('WPF','XAML (Property/collection syntax, Namespaces, markup extensions, type converters,  attached properties)\nWPF Core (Events, styles, resources, binding, control/data templates, standard controls, animation, triggers, custom/user controls, layout model, threading model, behaviors)\nWPF Frameworks (Prism, Chinch, MVVM Light)'),
('OOP','OOA/P concepts, Design Patterns, UML'),
('DB','External keys, normalization, triggers, stored procedures, JPA, Hybernate, ORM'),
('Web Basics','HTTP protocol, Servlets, JSP, JAX-RS, JAX-WS'),
('Build Tools','Provide a goal for this topic'),
('Version Control','Provide a goal for this topic'),
('Unit Tests','Mocks/fakes, IoC pattern, Lifetime managers, TDD, BDD'),
('NoSQL','Provide a goal for this topic'),
('BigData','Provide a goal for this topic'),
('Cloud','Provide a goal for this topic'),
('SDLC','Provide a goal for this topic'),
('Security','Provide a goal for this topic');

DROP TABLE questions;

CREATE TABLE questions
(
    id INTEGER PRIMARY KEY AUTOINCREMENT,

    seniority_level_id INTEGER NOT NULL,

    topic_id INTEGER NOT NULL,

    question_text TEXT NOT NULL,

    suggested_answer TEXT NOT NULL,

    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY(topic_id)
        REFERENCES question_topics(id),

    FOREIGN KEY(seniority_level_id)
        REFERENCES seniority_levels(id)
);
