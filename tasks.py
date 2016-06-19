from invoke import run, task

POT_PATH = "locale/messages.pot"


@task
def trans_extract():
    run(
        'pybabel extract --output %s ./' % POT_PATH,
        pty=True
    )


@task
def trans_init(lang):
    run(
        'pybabel init -i %s -d . -l %s' % (POT_PATH, lang),
        pty=True
    )


@task
def trans_update():
    run(
        'pybabel update -i %s -d ./locale' % POT_PATH,
        pty=True
    )


@task
def trans_compile():
    run('pybabel compile -f -d ./locale')
