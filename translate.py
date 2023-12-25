import argparse
import openai
import tiktoken


def translate_markdown(api_key, model, base_url, sysprompt, input_file):
    # 设置 OpenAI API 密钥
    openai.api_key = api_key
    openai.base_url = base_url

    # 读取 Markdown 文件
    with open(input_file, 'r', encoding='utf-8') as file:
        content = file.read()
    
    prompt_template = f"""
以下是一段英文技术文档。请将其翻译成中文，注意 system prompt 中的要求：{content}
"""
    messages = [
            {"role": "system", "content": sysprompt},
            {"role": "user", "content": prompt_template}
        ]
    # 调用 OpenAI API 进行翻译
    stream = openai.chat.completions.create(
        model=model,
        messages=messages,
        stream=True,
        max_tokens=4096,
    )
    response = ""
    for chunk in stream:
        if chunk.choices[0].delta.content is not None:
          print(chunk.choices[0].delta.content, end="")
          response += chunk.choices[0].delta.content
    # overwrite the original file
    with open(input_file, 'w', encoding='utf-8') as file:
        file.write(response)

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Translate Markdown files using OpenAI API.")
    parser.add_argument("-k", "--api-key", required=True, help="OpenAI API key")
    parser.add_argument("-m", "--model", default="gpt-3.5-turbo-16k", help="Model to be used for translation. gpt-3.5-turbo-16k and gpt-4-1106-preview are recommended.")
    parser.add_argument("-b", "--base-url", default="https://api.openai.com/v1/", help="Base URL for the OpenAI API")
    parser.add_argument("-s", "--sysprompt", default="""
请结合上下文，根据以下通用指南将英文技术文档翻译成中文, 只返回译文：
1. 使用标准技术术语，对于常用的技术术语保持英文原词，不进行翻译。
2. 考虑到中文的表达习惯，转换语序，补充句子成分，使句子符合中文语境的流畅性。
3. 根据上下文确定专有名词、多义词的翻译，专业名词或无直接中文对应词汇保留原文，并在可能的地方提供注释说明。
4. 翻译的时候应确保文本流畅易读，保持原始信息的完整。
""", help="System prompt to be used for translation")
    parser.add_argument("-f", "--file", required=True, help="Path to the Markdown file to be translated")

    args = parser.parse_args()

    translate_markdown(args.api_key, args.model, args.base_url, args.sysprompt, args.file)
