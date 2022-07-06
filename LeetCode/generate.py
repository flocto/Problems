import requests

query = "\n    query questionOfToday {\n  activeDailyCodingChallengeQuestion {\n    date\n    userStatus\n    link\n    question {\n      acRate\n      difficulty\n      freqBar\n      frontendQuestionId: questionFrontendId\n      isFavor\n      paidOnly: isPaidOnly\n      status\n      title\n      titleSlug\n      hasVideoSolution\n      hasSolution\n      topicTags {\n        name\n        id\n        slug\n      }\n    }\n  }\n}\n    "
variables = {}

daily_question = requests.post(
    "https://leetcode.com/graphql",
    json={"query": query, "variables": variables},
    headers={"Content-Type": "application/json"},
).content.decode("utf-8")

titleSlug = daily_question.split("titleSlug\":\"")[1].split("\"")[0]
link = "https://leetcode.com/problems/" + titleSlug

operationName = "questionData"
query = "query questionData($titleSlug: String!) {\n  question(titleSlug: $titleSlug) {\n    questionId\n    questionFrontendId\n    boundTopicId\n    title\n    titleSlug\n    content\n    translatedTitle\n    translatedContent\n    isPaidOnly\n    difficulty\n    likes\n    dislikes\n    isLiked\n    similarQuestions\n    exampleTestcases\n    categoryTitle\n    contributors {\n      username\n      profileUrl\n      avatarUrl\n      __typename\n    }\n    topicTags {\n      name\n      slug\n      translatedName\n      __typename\n    }\n    companyTagStats\n    codeSnippets {\n      lang\n      langSlug\n      code\n      __typename\n    }\n    stats\n    hints\n    solution {\n      id\n      canSeeDetail\n      paidOnly\n      hasVideoSolution\n      paidOnlyVideo\n      __typename\n    }\n    status\n    sampleTestCase\n    metaData\n    judgerAvailable\n    judgeType\n    mysqlSchemas\n    enableRunCode\n    enableTestMode\n    enableDebugger\n    envInfo\n    libraryUrl\n    adminUrl\n    challengeQuestion {\n      id\n      date\n      incompleteChallengeCount\n      streakCount\n      type\n      __typename\n    }\n    __typename\n  }\n}\n"
variables = {"titleSlug": titleSlug}

question_data = requests.post(
    "https://leetcode.com/graphql",
    json={"operationName": operationName, "query": query, "variables": variables},
    headers={"Content-Type": "application/json"},
).content.decode("utf-8")

frontend_id = question_data.split("questionFrontendId\":\"")[1].split("\"")[0]
title = question_data.split("title\":\"")[1].split("\"")[0]
name = frontend_id+". "+title

# hacky but it works :)
import os

os.makedirs(name, exist_ok=True)
with open(name+"/README.md", "w") as f:
    f.write("Problem located at " + link)
    f.write("\n\n")
    f.write("## Notes\n- Did not create an entire project as Rust can be run remotely from leetcode.")
    f.write("\n\n")
    f.write("# Solution\n")

with open(name+"/main.rs", "w") as f:
    pass

print("Created files for " + name)
print("Site: " + link)