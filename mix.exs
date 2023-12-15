defmodule BreezeRt.MixProject do
  use Mix.Project

  def project do
    [
      app: :breeze_rt,
      version: "0.1.0",
      elixir: "~> 1.15",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger],
      mod: {BreezeRt.Application, []}
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:rustler, "~> 0.30.0"},
      {:thousand_island, "~> 1.0"}
    ]
  end
end
