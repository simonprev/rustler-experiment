defmodule EnchantTest do
  use ExUnit.Case
  doctest Enchant

  test "greets the world" do
    assert Enchant.hello() == :world
  end
end
