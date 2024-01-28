<?php

namespace Neoncitylights\ExampleLibrary\Tests;

use Neoncitylights\ExampleLibrary\Dog;
use PHPUnit\Framework\TestCase;

/**
 * @coversDefaultClass Neoncitylights\ExampleLibrary\Dog
 * @uses Neoncitylights\ExampleLibrary\Animal
 */
class DogTest extends TestCase {
	/**
	 * @covers ::getName
	 */
	public function testGetName(): void {
		$this->assertEquals( 'Ralph', ( new Dog( 'Ralph', 4.0 ) )->getName() );
	}

	/**
	 * @covers ::getSound
	 */
	public function testGetSound(): void {
		$this->assertEquals( 'Woof!', ( new Dog( 'Charlie', 5.0 ) )->getSound() );
	}

	/**
	 * @covers ::getType
	 */
	public function testGetType(): void {
		$this->assertEquals( 'Dog', ( new Dog( 'Buddy', 3.2 ) )->getType() );
	}

	/**
	 * @covers ::getAge
	 */
	public function testGetAge(): void {
		$this->assertEqualsWithDelta( 8.7, ( new Dog( 'Waffles', 8.7 ) )->getAge(), 0.0001 );
	}
}
