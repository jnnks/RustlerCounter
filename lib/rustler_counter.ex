defmodule RustlerCounter.Native do
  use Rustler, otp_app: :rustler_counter, crate: :nativecounter

  def new(_count), do: :erlang.nif_error(:nif_not_loaded)
  def bump(_counter), do: :erlang.nif_error(:nif_not_loaded)
end

defmodule RustlerCounter do

  def new(count), do: RustlerCounter.Native.new(count)
  def bump(counter), do: RustlerCounter.Native.bump(counter)
end
