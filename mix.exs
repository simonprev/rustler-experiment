defmodule Enchant.MixProject do
  use Mix.Project

  @version "0.1.0"

  def project do
    [
      app: :enchant,
      version: @version,
      compilers: [:rustler] ++ Mix.compilers(),
      elixir: "~> 1.9",
      rustler_crates: [enchant_elixir: []],
      start_permanent: Mix.env() == :prod,
      source_url: "https://github.com/simonprev/enchant",
      docs: [
        source_ref: "v#{@version}",
        formatters: ["html", "epub"]
      ],
      package: package(),
      deps: deps()
    ]
  end

  def application do
    [
      extra_applications: [:logger]
    ]
  end

  defp deps do
    [
      {:rustler, "~> 0.21"},
      {:ex_doc, "~> 0.20", only: :dev}
    ]
  end

  defp package do
    [
      description: "Elixir bindings for enchant",
      files: [
        "lib",
        ".formatter.exs"
      ],
      maintainers: [
        "Simon Prévost"
      ],
      licenses: ["MIT"],
      links: %{
        GitHub: "https://github.com/simonprev/enchant"
      }
    ]
  end
end
