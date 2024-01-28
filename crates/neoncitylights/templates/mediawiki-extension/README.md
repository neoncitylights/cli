# MediaWiki extension repository template

This repository template is designed to allow quickly building open-source MediaWiki extensions within the ecosystem. It comes with everything that's needed to help ensure the extension follows [MediaWiki's best practices](https://www.mediawiki.org/wiki/Best_practices_for_extensions) and [coding conventions](https://www.mediawiki.org/wiki/Manual:Coding_conventions).

## Roadmap

- [ ] Show examples of creating a [special page](https://www.mediawiki.org/wiki/Manual:Special_pages)
- [ ] Show examples of using [MediaWiki hooks](https://www.mediawiki.org/wiki/Manual:Hooks)
- [ ] Show examples of creating a [MediaWiki Action API](https://www.mediawiki.org/wiki/API:Main_page) module
- [ ] Show examples of creating a MediaWiki REST API endpoint
- [ ] Show examples of using [ResourceLoader](https://www.mediawiki.org/wiki/ResourceLoader) (MediaWiki's bundler system, comparable to software like Webpack, esbuild, and Rollup)
- [ ] Show examples of using PHPUnit with MediaWiki's core test infrastructure
- [ ] Integrate GitHub Actions for CI/CD

## Get started

### Prerequisites

First, make sure you have a local MediaWiki instance running. You can do this in a few ways:

- Installing directly on your machine [[MacOS](https://www.mediawiki.org/wiki/Manual:Running_MediaWiki_on_macOS), [Windows](https://www.mediawiki.org/wiki/Manual:Running_MediaWiki_on_Windows)/[WSL](https://www.mediawiki.org/wiki/Manual:Running_MediaWiki_on_Windows_Subsystem_for_Linux), [Debian/Ubuntu](https://www.mediawiki.org/wiki/Special:MyLanguage/Manual:Running_MediaWiki_on_Debian_or_Ubuntu), [others...](https://www.mediawiki.org/wiki/Manual:OS_specific_help)]
- [Installing with Vagrant](https://www.mediawiki.org/wiki/MediaWiki-Vagrant) [[GitHub repo (mirror)](https://github.com/wikimedia/mediawiki-vagrant), [Gerrit repo (source)](https://gerrit.wikimedia.org/g/mediawiki/vagrant)]: automatically provisions a virtual machine (via VirtualBox) with a MediaWiki instance and all of its dependencies
- [Installing with MediaWiki-Docker](https://www.mediawiki.org/wiki/MediaWiki-Docker) [[Docker Hub](https://hub.docker.com/_/mediawiki), [repo](https://github.com/wikimedia/mediawiki-docker)]: Runs MediaWiki in a Docker container

### Creating a new repository

#### GitHub UI

You can create a new repository based on this template by clicking the "Use this template" button in the top-right corner of this page.

#### Terminal

You can run the following command below with the [GitHub CLI](https://cli.github.com/). Some notes:

- Replace placeholder with name of your extension in upper CamelCase
- Configure your repository's visibility with `--public`, `--private`, or `--internal`

```shell
gh repo create {{extension-uc}} --clone --template neoncitylights/mediawiki-extension-template
```

### Cookiecutter stuff

Using your favorite text editor or IDE, find-and-replace the following placeholders:

- `{{author}}`: Your own username/nickname, name, or organization
- `{{extension-uc}}`, `ExtensionUc`: Name of your extension in upper CamelCase
- `{{extension-lc}}`: Name of your extension in lowercase

### Running the extension locally

To get started, run the following commands in your terminal. This assumes you're in the **root directory** of your MediaWiki extension.

```shell
npm install      # Install Node.js developer dependencies
composer install # Install Composer developer dependencies
```

Then, in your MediaWiki's instance containing `LocalSettings.php`, add the following line at the bottom:

```php
wfLoadExtension( 'BoilerPlate' ); // Replace "BoilerPlate" with the name of your extension
```

### Confirm the extension is working

To confirm, visit `Special:Version` and `Special:HelloWorld`! E.g If you're running on port 8080, visit these links:

- `http://localhost:8080/index.php?title=Special:Version`
- `http://localhost:8080/index.php?title=Special:HelloWorld`

## Setup and configure

| Tool | Files | Links |
| ---- | ----- | ----- |
| MediaWiki extension configuration | [`extension.json`](./extension.json) | [docs](https://www.mediawiki.org/wiki/Manual:Extension.json), [schema reference](https://www.mediawiki.org/wiki/Manual:Extension.json/Schema) |
| NPM configuration | [`package.json`](./package.json) | [docs](https://docs.npmjs.com/cli/v9/configuring-npm/package-json), [website](https://www.npmjs.com/), [repo](https://github.com/npm/cli) |
| Composer configuration | [`composer.json`](./composer.json) | [docs](https://getcomposer.org/doc/04-schema.md), [website](https://getcomposer.org/), [repo](https://github.com/composer/composer) |
| Localization and translated messages | [`i18n/`](./i18n/) | [docs](https://www.mediawiki.org/wiki/Localisation) |
| Gruntfile (JS task runner) | [`Gruntfile.js`](./Gruntfile.js) | [docs](https://gruntjs.com/configuring-tasks), [website](https://gruntjs.com/), [repo](https://github.com/gruntjs/grunt) |
| ESLint (JS linter) | [`.eslintrc.json`](./.eslintrc.json) | [docs](https://eslint.org/docs/latest/use/configure/), [website](https://eslint.org/), [repo](https://github.com/eslint/eslint) |
| Stylelint (CSS linter) | [`.stylelintrc.json`](./.stylelintrc.json) | [docs](https://stylelint.io/docs/user-guide/configuration), [website](https://stylelint.io/), [repo](https://github.com/stylelint/stylelint) |
| PHPCS (PHP linter) | [`phpcs.xml`](./phpcs.xml) | [repo](https://github.com/squizlabs/PHP_CodeSniffer) |
| Phan (PHP static analysis) | [`.phan/config.php`](./.phan/config.php) | [mw.org docs](https://www.mediawiki.org/wiki/Continuous_integration/Phan), [phan repo](https://github.com/phan/phan/), [mw plugin repo](https://gerrit.wikimedia.org/g/mediawiki/tools/phan/SecurityCheckPlugin)
| PHPUnit (PHP unit testing framework) | [`phpunit.xml.dist`](./phpunit.xml.dist) | [docs](https://docs.phpunit.de/en/10.2/configuration.html), [website](https://phpunit.de/), [repo](https://github.com/sebastianbergmann/phpunit) |
| Dependabot (dependency updates) | [`.github/dependabot.yml`](./.github/dependabot.yml) | [docs](https://docs.github.com/en/code-security/dependabot/dependabot-version-updates/configuration-options-for-the-dependabot.yml-file) |

## Notes

### Setting up your local code editor/IDE environment

**Intellisense** (or intelligent code completion) is supported by many code editors and IDEs. To make sure you have this enabled, you'll need to make sure you have your MediaWiki directory open, to avoid warnings like `Undefined type 'SpecialPage'`. There's also a few setups I recommend below, however you can use whatever works best for you. :-)

- [Visual Studio Code](https://code.visualstudio.com/) (by Microsoft, free, open-source), with [PHP Intelephense](https://marketplace.visualstudio.com/items?itemName=bmewburn.vscode-intelephense-client), [PHP Debug](https://marketplace.visualstudio.com/items?itemName=xdebug.php-debug) extensions installed
- [PHPStorm](https://www.jetbrains.com/phpstorm/) (by JetBrains, closed-source), with the [MediaWiki Support](https://plugins.jetbrains.com/plugin/7439-mediawiki-support) plugin installed. This IDE has various pricing plans, but is [free for students/teachers](https://www.jetbrains.com/community/education/#students). MediaWiki.org also has an article about [requesting a JetBrains license key](https://www.mediawiki.org/wiki/JetBrains_IDEs) by contacting one of the account administrators.

### Configuring the minimum-supported PHP version

> **Note**
> Please also keep in mind whether or not the minimum MediaWiki version you're running against supports that specific PHP version.

There's a few different files you'll need to change.

- [`composer.json`](./composer.json): Change the `require.php` key. An example for supporting PHP 7.4 or greater:

  ```json
  "require": {
    "php": ">=7.4"
  }
  ```

- [`extension.json`](./extension.json): Change the [`requires.platform.php`](https://www.mediawiki.org/wiki/Manual:Extension.json/Schema#platform) key. To support PHP 7.4 or greater:

  ```json
  "requires": {
    "platform": {
      "php": ">= 7.4"
    }
  }
  ```

- [`.github/workflows/ci.yml`](./.github/workflows/ci.yml): Change the `php` key in the `matrix` section.
- [`.phan/config.php`](./.phan/config.php): Change the `minimum_target_php_version` key.
- [`README.md`](./README.md): Change your main, **user-facing** documentation where the system requirements section is to reflect the minimum PHP version.

### Configuring MediaWiki extension dependencies and minimum-supported MediaWiki version

#### `extension.json`

In the MediaWiki ecosystem, we use `composer.json` mostly for developer dependencies. However, we use `extension.json` overall for registering MediaWiki extensions (and `skin.json` for registering MediaWiki skins).

If your extension depends on other MediaWiki extensions to work (e.g BetaFeatures, Echo, etc.), you can add them to the `extension.json` file via the [`requires.extensions`](https://www.mediawiki.org/wiki/Manual:Extension.json/Schema#requires) key.

> **Note**
> Unlike other ecosystems where it's common to use semantic versioning (e.g. `^1.2.3`), MediaWiki extensions use a different versioning scheme, and different extensions will vary in their [compatibility policy](https://www.mediawiki.org/wiki/Compatibility#MediaWiki_extensions). Some special notes:
>
> - You can use any version syntax that [Composer supports](https://getcomposer.org/doc/articles/versions.md).
> - A wildcard (`*`) means that any version of the extension is acceptable.
> - You can view an extension's compatibility policy by visiting their page on [MediaWiki.org](https://www.mediawiki.org/wiki/Category:Extensions_by_compatibility_policy).

```json
{
	"requires": {
		"MediaWiki": ">= 1.39",
		"extensions": {
			"BetaFeatures": "*",
			"Echo": "*"
		}
	}
}
```

#### `README.md`

You should also update your `README.md` instructions on how to install your extension, e.g the instructions for editing `LocalSettings.php` might look like:

```php
wfLoadExtension( 'BetaFeatures' );
wfLoadExtension( 'Echo' );
wfLoadExtension( 'BoilerPlate' );
```
