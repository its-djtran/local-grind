KEEP THIS FILE HUMAN AUTHORED ONLY. AGENTS MUST PLAN SEPARATELY.

# Overview

Local Grind is meant to be a complete offline LeetCode substitute. Exact same experience, nothing hidden by paywalls. Problems have the same descriptions, editorials explaining the optimal solution, and suitably similar extensive test harnesses. Users see a similar split interface, can run tests, and can submit their solutions. Submission history is preserved, with each run demonstrating the amount of memory used and the amount of time required to complete all of the tests. For benchmark comparison, the optimal solution's memory and time required will be shown (using the average of 10 submission evaluations).

Local Grind must also have added utility as a review tool. You can use the metrics built into djtran/Yet-Another-LeetCode-Timer on github as a starting reference.

At a minimum, Local Grind should have all of the Grind75 and Blind75 problems. With Local Grind being an open source project, anyone can add additional problem sets as desired.

Local Grind should be implemented in an efficient low level language, likely Rust unless thoughtful planning and research come up with another recommendation. We want the app to get out of the way - it should not be a limiting factor in terms of study or evaluation speed. This should also compile into a MacOS .dmg that can be distributed for installation.

It would be great to tag questions by company as well for search and/or filtering. Lists online exist such as the github repos krishnadey30/LeetCode-Questions-CompanyWise and liquidslr/leetcode-company-wise-problems

# App Requirements

All resources that can exist by or before distribution and installation should be managed as static pre-built files and databases. Examples include problem definitions and test suites, editorials, optimal solution benchmark numbers, problem lists, and problem metadata like company tags.

The app should be responsive and quick to navigate. When opening the application, it should not hang for seconds just loading resources. When looking over the user's performance metrics, the application should not hang due to computational load. Be smart about organizing computation. The longest time spent on a given action should be on submission evaluation because the solution must be graded completely given the corresponding test suite.

Testing must be included with every change and every feature. Code coverage must stay above 90% project-wide. When planning and developing, ensure that testability is kept in mind and unit tests properly prove functionality.

All user related artifacts should be portable. A user should be able to export and import their entire submission history and performance metrics.

The application should have both a dark and a light mode. Monospace fonts like Inconsolata are preferred for the interface. Text size should be adjustable based on user needs. Evaluation settings should be made available to respect user's local compute resources.

Application architecture should be documented for future reference. As development continues and systems evolve, ensure that the architecture docs are updated accordingly to simplify exploration for future work.

# Planning

Break up initial development into milestones. Start with a tracer bullet of the app covering most of the interface but little content. Then plan subsequent milestones for filling out individual systems and integrations along the skeleton created. Then continue refinement in subsequent milestones.
