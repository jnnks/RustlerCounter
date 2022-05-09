defmodule RustlerCounterTest do
  use ExUnit.Case
  doctest RustlerCounter

  test "count to 10" do
    rc = RustlerCounter.new(0)
    1..10 |> Enum.each(fn i ->
      count = RustlerCounter.bump(rc)
      assert i == count
    end)
  end
end
