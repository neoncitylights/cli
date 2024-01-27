<?php

namespace MediaWiki\Extension\ExtensionUc\Specials;

use SpecialPage;

/**
 * @license MIT
 */
class SpecialHelloWorld extends SpecialPage {
	/**
	 * @inheritDoc
	 */
	public function __construct() {
		parent::__construct( 'HelloWorld' );
	}

	/**
	 * @inheritDoc
	 */
	public function execute( $sub ) {
		$this->setHeaders();
		$this->outputHeader();
		$this->getOutput()->addWikiMsg( '{{extension-lc}}-helloworld' );
	}
}
