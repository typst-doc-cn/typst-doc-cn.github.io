import jinja2
import json
import os
import shutil


def render_jinja_html(template_loc, file_name, **context):
    return jinja2.Environment(
        loader=jinja2.FileSystemLoader(template_loc + '/')
    ).get_template(file_name).render(context)


type2class_map = {
    'none': 'pill-kw',
    'auto': 'pill-kw',
    'function': 'pill-fn',
    'string': 'pill-str',
    'content': 'pill-con',
    'color': 'pill-col',
    'boolean': 'pill-bool',
    'integer': 'pill-num',
    'ratio': 'pill-num',
    'length': 'pill-num',
    'relative length': 'pill-num',
    'float': 'pill-num',
    'angle': 'pill-num',
    'fraction': 'pill-num',
}


def type2class(type):
    return type2class_map.get(type, 'pill-obj')


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
        with open('./dist' + page['route'] + 'index.html', 'w', encoding='utf-8') as f:
            f.write(render_jinja_html('./templates/', page['body']['kind'] + '_template.html',
                    docs=docs, path=path, prev=prev, next=next, type2class=type2class, **page))
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
        for page in docs:
            dfs(page, docs)
        for page in docs:
            render_to_files(page, docs, [page])
