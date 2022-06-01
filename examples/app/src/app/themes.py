import plotly.graph_objects as go

theme_light = go.layout.Template(  # type: ignore
    {
        "data": {
            "bar": [
                {
                    "error_x": {"color": "#2a3f5f"},
                    "error_y": {"color": "#2a3f5f"},
                    "marker": {"line": {"color": "white", "width": 0.5}},
                    "type": "bar",
                }
            ],
            "barpolar": [{"marker": {"line": {"color": "white", "width": 0.5}}, "type": "barpolar"}],
            "carpet": [
                {
                    "aaxis": {
                        "endlinecolor": "#2a3f5f",
                        "gridcolor": "#C8D4E3",
                        "linecolor": "#C8D4E3",
                        "minorgridcolor": "#C8D4E3",
                        "startlinecolor": "#2a3f5f",
                    },
                    "baxis": {
                        "endlinecolor": "#2a3f5f",
                        "gridcolor": "#C8D4E3",
                        "linecolor": "#C8D4E3",
                        "minorgridcolor": "#C8D4E3",
                        "startlinecolor": "#2a3f5f",
                    },
                    "type": "carpet",
                }
            ],
            "choropleth": [{"colorbar": {"outlinewidth": 0, "ticks": ""}, "type": "choropleth"}],
            "contour": [
                {
                    "colorbar": {"outlinewidth": 0, "ticks": ""},
                    "colorscale": [
                        [0.0, "#0d0887"],
                        [0.1111111111111111, "#46039f"],
                        [0.2222222222222222, "#7201a8"],
                        [0.3333333333333333, "#9c179e"],
                        [0.4444444444444444, "#bd3786"],
                        [0.5555555555555556, "#d8576b"],
                        [0.6666666666666666, "#ed7953"],
                        [0.7777777777777778, "#fb9f3a"],
                        [0.8888888888888888, "#fdca26"],
                        [1.0, "#f0f921"],
                    ],
                    "type": "contour",
                }
            ],
            "contourcarpet": [{"colorbar": {"outlinewidth": 0, "ticks": ""}, "type": "contourcarpet"}],
            "heatmap": [
                {
                    "colorbar": {"outlinewidth": 0, "ticks": ""},
                    "colorscale": [
                        [0.0, "#0d0887"],
                        [0.1111111111111111, "#46039f"],
                        [0.2222222222222222, "#7201a8"],
                        [0.3333333333333333, "#9c179e"],
                        [0.4444444444444444, "#bd3786"],
                        [0.5555555555555556, "#d8576b"],
                        [0.6666666666666666, "#ed7953"],
                        [0.7777777777777778, "#fb9f3a"],
                        [0.8888888888888888, "#fdca26"],
                        [1.0, "#f0f921"],
                    ],
                    "type": "heatmap",
                }
            ],
            "heatmapgl": [
                {
                    "colorbar": {"outlinewidth": 0, "ticks": ""},
                    "colorscale": [
                        [0.0, "#0d0887"],
                        [0.1111111111111111, "#46039f"],
                        [0.2222222222222222, "#7201a8"],
                        [0.3333333333333333, "#9c179e"],
                        [0.4444444444444444, "#bd3786"],
                        [0.5555555555555556, "#d8576b"],
                        [0.6666666666666666, "#ed7953"],
                        [0.7777777777777778, "#fb9f3a"],
                        [0.8888888888888888, "#fdca26"],
                        [1.0, "#f0f921"],
                    ],
                    "type": "heatmapgl",
                }
            ],
            "histogram": [{"marker": {"colorbar": {"outlinewidth": 0, "ticks": ""}}, "type": "histogram"}],
            "histogram2d": [
                {
                    "colorbar": {"outlinewidth": 0, "ticks": ""},
                    "colorscale": [
                        [0.0, "#0d0887"],
                        [0.1111111111111111, "#46039f"],
                        [0.2222222222222222, "#7201a8"],
                        [0.3333333333333333, "#9c179e"],
                        [0.4444444444444444, "#bd3786"],
                        [0.5555555555555556, "#d8576b"],
                        [0.6666666666666666, "#ed7953"],
                        [0.7777777777777778, "#fb9f3a"],
                        [0.8888888888888888, "#fdca26"],
                        [1.0, "#f0f921"],
                    ],
                    "type": "histogram2d",
                }
            ],
            "histogram2dcontour": [
                {
                    "colorbar": {"outlinewidth": 0, "ticks": ""},
                    "colorscale": [
                        [0.0, "#0d0887"],
                        [0.1111111111111111, "#46039f"],
                        [0.2222222222222222, "#7201a8"],
                        [0.3333333333333333, "#9c179e"],
                        [0.4444444444444444, "#bd3786"],
                        [0.5555555555555556, "#d8576b"],
                        [0.6666666666666666, "#ed7953"],
                        [0.7777777777777778, "#fb9f3a"],
                        [0.8888888888888888, "#fdca26"],
                        [1.0, "#f0f921"],
                    ],
                    "type": "histogram2dcontour",
                }
            ],
            "mesh3d": [{"colorbar": {"outlinewidth": 0, "ticks": ""}, "type": "mesh3d"}],
            "parcoords": [{"line": {"colorbar": {"outlinewidth": 0, "ticks": ""}}, "type": "parcoords"}],
            "pie": [{"automargin": True, "type": "pie"}],
            "scatter": [{"marker": {"colorbar": {"outlinewidth": 0, "ticks": ""}}, "type": "scatter"}],
            "scatter3d": [
                {
                    "line": {"colorbar": {"outlinewidth": 0, "ticks": ""}},
                    "marker": {"colorbar": {"outlinewidth": 0, "ticks": ""}},
                    "type": "scatter3d",
                }
            ],
            "scattercarpet": [{"marker": {"colorbar": {"outlinewidth": 0, "ticks": ""}}, "type": "scattercarpet"}],
            "scattergeo": [{"marker": {"colorbar": {"outlinewidth": 0, "ticks": ""}}, "type": "scattergeo"}],
            "scattergl": [{"marker": {"colorbar": {"outlinewidth": 0, "ticks": ""}}, "type": "scattergl"}],
            "scattermapbox": [{"marker": {"colorbar": {"outlinewidth": 0, "ticks": ""}}, "type": "scattermapbox"}],
            "scatterpolar": [{"marker": {"colorbar": {"outlinewidth": 0, "ticks": ""}}, "type": "scatterpolar"}],
            "scatterpolargl": [
                {
                    "marker": {"colorbar": {"outlinewidth": 0, "ticks": ""}},
                    "type": "scatterpolargl",
                }
            ],
            "scatterternary": [
                {
                    "marker": {"colorbar": {"outlinewidth": 0, "ticks": ""}},
                    "type": "scatterternary",
                }
            ],
            "surface": [
                {
                    "colorbar": {"outlinewidth": 0, "ticks": ""},
                    "colorscale": [
                        [0.0, "#0d0887"],
                        [0.1111111111111111, "#46039f"],
                        [0.2222222222222222, "#7201a8"],
                        [0.3333333333333333, "#9c179e"],
                        [0.4444444444444444, "#bd3786"],
                        [0.5555555555555556, "#d8576b"],
                        [0.6666666666666666, "#ed7953"],
                        [0.7777777777777778, "#fb9f3a"],
                        [0.8888888888888888, "#fdca26"],
                        [1.0, "#f0f921"],
                    ],
                    "type": "surface",
                }
            ],
            "table": [
                {
                    "cells": {"fill": {"color": "#EBF0F8"}, "line": {"color": "white"}},
                    "header": {"fill": {"color": "#C8D4E3"}, "line": {"color": "white"}},
                    "type": "table",
                }
            ],
        },
        "layout": {
            "annotationdefaults": {"arrowcolor": "#2a3f5f", "arrowhead": 0, "arrowwidth": 1},
            "coloraxis": {"colorbar": {"outlinewidth": 0, "ticks": ""}},
            "colorscale": {
                "diverging": [
                    [0, "#8e0152"],
                    [0.1, "#c51b7d"],
                    [0.2, "#de77ae"],
                    [0.3, "#f1b6da"],
                    [0.4, "#fde0ef"],
                    [0.5, "#f7f7f7"],
                    [0.6, "#e6f5d0"],
                    [0.7, "#b8e186"],
                    [0.8, "#7fbc41"],
                    [0.9, "#4d9221"],
                    [1, "#276419"],
                ],
                "sequential": [
                    [0.0, "#0d0887"],
                    [0.1111111111111111, "#46039f"],
                    [0.2222222222222222, "#7201a8"],
                    [0.3333333333333333, "#9c179e"],
                    [0.4444444444444444, "#bd3786"],
                    [0.5555555555555556, "#d8576b"],
                    [0.6666666666666666, "#ed7953"],
                    [0.7777777777777778, "#fb9f3a"],
                    [0.8888888888888888, "#fdca26"],
                    [1.0, "#f0f921"],
                ],
                "sequentialminus": [
                    [0.0, "#0d0887"],
                    [0.1111111111111111, "#46039f"],
                    [0.2222222222222222, "#7201a8"],
                    [0.3333333333333333, "#9c179e"],
                    [0.4444444444444444, "#bd3786"],
                    [0.5555555555555556, "#d8576b"],
                    [0.6666666666666666, "#ed7953"],
                    [0.7777777777777778, "#fb9f3a"],
                    [0.8888888888888888, "#fdca26"],
                    [1.0, "#f0f921"],
                ],
            },
            "legend": {"orientation": "h", "xanchor": "right", "x": 1.0, "y": 1.02},
            "margin": {"autoexpand": True, "t": 42, "b": 69, "r": 42},
            "colorway": [
                "#119dff",
                "#66c2a5",
                "#fc8d62",
                "#e78ac3",
                "#a6d854",
                "#ffd92f",
                "#e5c494",
                "#b3b3b3",
            ],
            # "colorway": [
            #     "#636efa",
            #     "#EF553B",
            #     "#00cc96",
            #     "#ab63fa",
            #     "#FFA15A",
            #     "#19d3f3",
            #     "#FF6692",
            #     "#B6E880",
            #     "#FF97FF",
            #     "#FECB52",
            # ],
            "font": {"color": "#2a3f5f"},
            "geo": {
                "bgcolor": "white",
                "lakecolor": "white",
                "landcolor": "white",
                "showlakes": True,
                "showland": True,
                "subunitcolor": "#C8D4E3",
            },
            "hoverlabel": {"align": "left", "namelength": -1},
            "hovermode": "closest",
            "mapbox": {"style": "light"},
            "paper_bgcolor": "white",
            "plot_bgcolor": "white",
            "polar": {
                "angularaxis": {"gridcolor": "#EBF0F8", "linecolor": "#EBF0F8", "ticks": ""},
                "bgcolor": "white",
                "radialaxis": {"gridcolor": "#EBF0F8", "linecolor": "#EBF0F8", "ticks": ""},
            },
            "scene": {
                "xaxis": {
                    "backgroundcolor": "white",
                    "gridcolor": "#DFE8F3",
                    "gridwidth": 2,
                    "linecolor": "#EBF0F8",
                    "showbackground": True,
                    "ticks": "",
                    "zerolinecolor": "#EBF0F8",
                },
                "yaxis": {
                    "backgroundcolor": "white",
                    "gridcolor": "#DFE8F3",
                    "gridwidth": 2,
                    "linecolor": "#EBF0F8",
                    "showbackground": True,
                    "ticks": "",
                    "zerolinecolor": "#EBF0F8",
                },
                "zaxis": {
                    "backgroundcolor": "white",
                    "gridcolor": "#DFE8F3",
                    "gridwidth": 2,
                    "linecolor": "#EBF0F8",
                    "showbackground": True,
                    "ticks": "",
                    "zerolinecolor": "#EBF0F8",
                },
            },
            "shapedefaults": {"line": {"color": "#2a3f5f"}},
            "ternary": {
                "aaxis": {"gridcolor": "#DFE8F3", "linecolor": "#A2B1C6", "ticks": ""},
                "baxis": {"gridcolor": "#DFE8F3", "linecolor": "#A2B1C6", "ticks": ""},
                "bgcolor": "white",
                "caxis": {"gridcolor": "#DFE8F3", "linecolor": "#A2B1C6", "ticks": ""},
            },
            "title": {"x": 0.05},
            "xaxis": {
                "automargin": True,
                "gridcolor": "#EBF0F8",
                "linecolor": "#EBF0F8",
                "ticks": "",
                "title": {"standoff": 15},
                "zerolinecolor": "#EBF0F8",
                "zerolinewidth": 2,
            },
            "yaxis": {
                "automargin": True,
                "gridcolor": "#EBF0F8",
                "linecolor": "#EBF0F8",
                "ticks": "",
                "title": {"standoff": 15},
                "zerolinecolor": "#EBF0F8",
                "zerolinewidth": 2,
            },
        },
    }
)
