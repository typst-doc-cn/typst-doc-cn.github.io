import jinja2
import json
import os
import shutil
import yaml


def str_presenter(dumper, data):
    if len(data.splitlines()) > 1:  # check for multiline string
        return dumper.represent_scalar('tag:yaml.org,2002:str', data, style='|')
    return dumper.represent_scalar('tag:yaml.org,2002:str', data)


yaml.add_representer(str, str_presenter)


def translate_with_yaml(page):
    if page['body']['kind'] in ['func', 'type', 'group']:
        assert page['route'].startswith('/docs/reference/')
        if page['body']['kind'] == 'group':
            path = page['route'][len('/docs/reference/'):].split('/')
        else:
            path = page['route'][len('/docs/reference/'):].split('/')[:-1]
        assert len(path) == 2
        # if docs/i18n/{path[0]} not exists, create it
        if not os.path.exists('docs/i18n/' + path[0]):
            os.mkdir('docs/i18n/' + path[0])
        # without quotes and with indent
        en_path = 'docs/i18n/' + path[0] + '/' + path[1] + '-en.yaml'
        zh_path = 'docs/i18n/' + path[0] + '/' + path[1] + '-zh.yaml'
        with open(en_path, 'w', encoding='utf-8') as f:
            yaml.dump(page, f, allow_unicode=True, default_flow_style=False,
                      indent=2, sort_keys=False, encoding='utf-8')
        if not os.path.exists(zh_path):
            with open(zh_path, 'w', encoding='utf-8') as f:
                yaml.dump(page, f, allow_unicode=True, default_flow_style=False,
                          indent=2, sort_keys=False, encoding='utf-8')
        if os.path.exists(zh_path):
            with open(zh_path, 'r', encoding='utf-8') as f:
                page = yaml.load(f, Loader=yaml.FullLoader)
    for i in range(len(page['children'])):
        page['children'][i] = translate_with_yaml(page['children'][i])
    return page


type2class_map = {
    'none': 'pill-kw',
    'auto': 'pill-kw',
    'function': 'pill-fn',
    'string': 'pill-str',
    'str': 'pill-str',
    'content': 'pill-con',
    'color': 'pill-col',
    'bool': 'pill-bool',
    'boolean': 'pill-bool',
    'integer': 'pill-num',
    'int': 'pill-num',
    'ratio': 'pill-num',
    'length': 'pill-num',
    'relative length': 'pill-num',
    'float': 'pill-num',
    'angle': 'pill-num',
    'fraction': 'pill-num',
}


def type2class(type):
    return type2class_map.get(type, 'pill-obj')

def gen_path(item):
    return ''.join([s + '.' for s in item['path']])


def render_jinja_html(template_loc, file_name, **context):
    return jinja2.Environment(
        loader=jinja2.FileSystemLoader(template_loc + '/')
    ).get_template(file_name).render(context)


if __name__ == '__main__':

    flattern_pages = []
    index = 0

    def dfs(page, docs):
        flattern_pages.append(page)
        for child in page['children']:
            dfs(child, docs)

    def render_to_files(page, docs, path):
        global index
        prev = flattern_pages[index - 1] if index > 0 else None
        next = flattern_pages[index +
                              1] if index < len(flattern_pages) - 1 else None
        if not os.path.exists('./dist' + page['route']):
            os.makedirs('./dist' + page['route'])
        with open('./dist' + page['route'] + ('/' if not page['route'].endswith('/') else '') + 'index.html', 'w', encoding='utf-8') as f:
            f.write(render_jinja_html('./templates/', page['body']['kind'] + '_template.html.j2',
                    docs=docs, path=path, prev=prev, next=next, type2class=type2class, gen_path=gen_path, **page))
        index += 1
        for child in page['children']:
            render_to_files(child, docs, path + [child])

    # cargo test --package typst-docs --lib -- tests::test_docs --exact --nocapture

    # clean dist
    if os.path.exists('./dist'):
        shutil.rmtree('./dist')

    # copy static to dist
    shutil.copytree('./static', './dist')

    # delete ./dist/assets/docs
    if os.path.exists('./dist/assets/docs'):
        shutil.rmtree('./dist/assets/docs')

    # copy assets/docs to dist/assets/docs
    shutil.copytree('./assets/docs', './dist/assets/docs')

    # load docs.json and render to files
    with open('./assets/docs.json', 'r', encoding='utf-8') as f:
        docs = json.load(f)
        # if docs/i18n not exists, create it
        if not os.path.exists('docs/i18n'):
            os.mkdir('docs/i18n')
        for i in range(len(docs)):
            docs[i] = translate_with_yaml(docs[i])
        for page in docs:
            dfs(page, docs)
        for page in docs:
            render_to_files(page, docs, [page])
