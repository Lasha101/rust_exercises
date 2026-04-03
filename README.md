Rust Exercises
After watching an introductory video course on Rust, I decided to acquire hands-on experience by using Brian P. Hogan's book, Exercises for Programmers: 57 Challenges to Develop Your Coding Skills. In this repository, I commit my solved exercises.

My Approach
The book provides a universal collection of tasks that can be solved with any general-purpose language. I use a specific learning method: using an LLM strictly as documentation.

Each time I open a conversation with the LLM, I provide the following strict instructions:

All answers must be short and without analogies.

Never provide partial or entire code solutions.

Do not provide any pseudo-code.

You may provide tool names (functions, methods, libraries).

For each provided tool, explain how to use it and provide a general, non-adapted example.

I then evaluate the suggested tools by asking the LLM the following questions:

Is this tool mandatory or optional?

If mandatory, why?

If optional, how can I resolve the problem without it?

Are there alternative tools to achieve the same purpose?

If yes, why use the current tool over the others?

What are the pros and cons of each?

I never copy and paste code from the LLM conversation into my implementation files.

Repository Structure
All exercises are located in the /src/bin directory.

Sometimes there are multiple solutions for the same problem. For example, exercise 5 has ex_5_1 and ex_5. The ex_5 version is intentionally over-engineered; I used interfaces (traits) to achieve the Open-Closed Principle (OCP).

Tech Stack
Language: Rust