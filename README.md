# Version Control System

Как git, только проще

## Поддерживаемые команды

`vcs init --path <directory path>` — создание репозитория по пути `path`
  
`vcs status` — выводит в терминал сообщение о текущем состоянии репозитория
  
`vcs commit --message <message>` — создание коммита с сообщением `message`

`vcs jump --commit <commit_hash>` — перенос репозитория в коммит с хэшом `commit_hash`
  
`vcs jump --branch <branch_name>` — перенос репозитория в последний коммит ветки `branch_name`

`vcs new_branch --name <branch_name>` — создание ветки с именем `branch_name`
  
`vcs merge --branch <branch_name>` — мердж ветки `branch_name` в `master` и последующее удаление всех её коммитов
