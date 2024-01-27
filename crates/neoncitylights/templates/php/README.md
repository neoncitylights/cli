# PHP Repository Template

> This template is based on my past history with developing MediaWiki software, and largely based on MediaWiki's own repository template for PHP projects.
>
> *However*, this project can be used for **any** PHP project, not just MediaWiki.
>
> I hope this template can be useful for you!

This repository makes it easier to develop a PHP application or library by providing a basic project structure, continuous integration (CI) configuration, and other tools.

## Features

- [x] [PHP 8.0](https://www.php.net/releases/8.0/en.php) support
- [x] *Unit testing* with [PHPUnit](https://github.com/sebastianbergmann/phpunit/) ([official website](https://phpunit.de/))
- [x] Validates PHP syntax with [php-parallel-lint](https://github.com/php-parallel-lint/PHP-Parallel-Lint)
- [x] Lint and auto-format PHP code with [PHP CodeSniffer](https://github.com/squizlabs/PHP_CodeSniffer) (PHPCS)
- [x] Security analysis with [Psalm](https://github.com/vimeo/psalm) ([official website](https://psalm.dev/))
- [x] Continuous integration with [GitHub Actions](https://github.com/features/actions)
- [x] auto-generated documentation with [phpDocumentor](https://www.phpdoc.org/), [GitHub Pages](https://docs.github.com/en/pages)
- [x] Remote development with [GitHub Codespaces](https://github.com/features/codespaces)
- [x] Composer dependency updates auto-managed with Dependabot

## Getting started

### Creating a new repository

#### GitHub UI

You can create a new repository based on this template by clicking the "Use this template" button in the top-right corner of this page.

#### Terminal

You can run the following command below with the [GitHub CLI](https://cli.github.com/). Some notes:

- Replace placeholder with name of your extension in upper CamelCase
- Configure your repository's visibility with `--public`, `--private`, or `--internal`

```shell
gh repo create {repo-name} --public --clone --template neoncitylights/php-template
```

### Cookiecutter stuff

Using your favorite text editor or IDE, find-and-replace the following placeholders:

- `{repo-name}`: Replace with the name of your repository
- `author/package`: Replace with your own author and package name.
- `neoncitylights/php-template`: Replace with the name of your GitHub repository.
- `Neoncitylights\ExampleLibrary`: Replace with your own namespace.
- `Neoncitylights\\ExampleLibrary\\`: Replace with your own namespace (escaped version using double backslashes).

Delete this `README.md`, and rename [`LIBRARY.md`](./LIBRARY.md) to `README.md`.

## Configure

| Tool | File | Documentation |
| ---- | ---- | ------------- |
| Composer package | [`composer.json`](./composer.json) | [docs](https://getcomposer.org/doc/04-schema.md), [website](https://getcomposer.org/), [repo](https://github.com/composer/composer) |
| PHPCS (formatter + linter) | [`.phpcs.xml`](./.phpcs.xml) | [docs](https://github.com/squizlabs/PHP_CodeSniffer/wiki/Advanced-Usage#using-a-default-configuration-file), [repo](https://github.com/squizlabs/PHP_CodeSniffer) |
| PHPUnit (unit testing) | [`.phpunit.xml.dist`](./.phpunit.xml.dist) | [docs](https://phpunit.readthedocs.io/en/10.1/configuration.html), [website](https://phpunit.de/), [repo](https://github.com/sebastianbergmann/phpunit) |
| PSalm (security analysis) | [`psalm.xml`](./psalm.xml) | [docs](https://psalm.dev/docs/running_psalm/configuration/), [website](https://psalm.dev/), [repo](https://github.com/vimeo/psalm) |
| phpdoc (documentation generator) | [`phpdoc.dist.xml`](./phpdoc.dist.xml) | [docs](https://docs.phpdoc.org/3.0/guide/references/configuration.html), [website](https://www.phpdoc.org/), [repo](https://github.com/phpdocumentor/phpdocumentor) |
| Dependabot | [`.github/dependabot.yml`](./.github/dependabot.yml) | [docs](https://docs.github.com/en/code-security/dependabot/dependabot-version-updates/configuration-options-for-the-dependabot.yml-file), [website](https://github.com/dependabot) |

## See also

- [Quick start to reStructuredText](https://docutils.sourceforge.io/docs/user/rst/quickstart.html) (`.rst`), markup language used for phpDocumentor

## License

This software is licensed under the MIT license ([`LICENSE-MIT`](./LICENSE) or <http://opensource.org/licenses/MIT>).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the MIT license, shall be licensed as above, without any additional terms or conditions.
