<?php

namespace Neoncitylights\ExampleLibrary;

/**
 * A cute dog!
 *
 * @license MIT
 */
class Dog implements IAnimal {
	private string $name;
	private float $age;

	public function __construct( string $name, float $age ) {
		$this->name = $name;
		$this->age = $age;
	}

	public function getName(): string {
		return $this->name;
	}

	public function getAge(): float {
		return $this->age;
	}

	public function getType(): string {
		return 'Dog';
	}

	public function getSound(): string {
		return 'Woof!';
	}
}
