defmodule BreezeRtTest do
  use ExUnit.Case
  doctest BreezeRt

  test "greets the world" do
    assert BreezeRt.hello() == :world
  end
end
