import json

import dash_antd as dadc
import plotly.express as px
import plotly.graph_objects as go
from dash import register_page  # type: ignore
from dash import Input, Output, State, callback, dcc, no_update

from app.context import ctx
from app.themes import theme_light

register_page(path="/", title="Explorer", icon="ControlOutlined")  # type: ignore


def controls():
    dataset_options = [
        {
            "label": ">".join(info.asset_key.path),
            "value": json.dumps(info.asset_key.path),
        }
        for info in ctx.fusion.list_datasets()
    ]
    return dadc.Select(
        id="dataset",
        options=dataset_options,
        style={"width": "100%"},
    )


def get_layout():
    return [
        dadc.Row(
            dadc.PageHeader(
                [
                    dadc.Space(
                        [
                            dadc.Select(
                                id="columns-x",
                                options=[],
                                style={"width": 250},
                            ),
                            dadc.Select(
                                id="columns-y",
                                options=[],
                                style={"width": 250},
                            ),
                        ]
                    ),
                    dadc.PageHeaderOperation(dadc.Button("Update Plot", id="update-plot", type="primary")),
                ],
                id="header",
                title="Dataset",
                ghost=False,
                style={
                    "width": "100%",
                    "height": 150,
                    "borderBottom": "1px solid rgba(0,0,0,.06)",
                },
            ),
            style={"height": "150px", "width": "100%"},
        ),
        dadc.Row(
            dcc.Graph(
                id="dataset-plot",
                figure=go.Figure(layout={"template": theme_light}),
                responsive=True,
                style={"height": "calc(100vh - 200px)", "width": "100%"},
            ),
            style={"height": "calc(100vh - 200px)", "width": "100%"},
        ),
    ]


def layout():
    return get_layout()


@callback(
    Output("header", "children"),
    Output("header", "title"),
    Input("dataset", "value"),
    prevent_initial_call=True,
)
def update_selects(asset_key: str):
    asset = ctx.get_dataset_client(asset_key)
    options = asset.column_options()
    meta = asset.client.get_metadata()
    controls = [
        dadc.Space(
            [
                dadc.Select(
                    id="columns-x",
                    options=options,
                    style={"width": 250},
                ),
                dadc.Select(
                    id="columns-y",
                    options=options,
                    style={"width": 250},
                ),
            ]
        ),
        dadc.PageHeaderOperation(dadc.Button("Update Plot", id="update-plot", type="primary")),
    ]

    if meta.is_versioned:
        controls = controls + [dadc.Tag("Versioned", color="green")]

    return controls, meta.name or asset.client.name


@callback(
    Output("dataset-plot", "figure"),
    Input("update-plot", "n_clicks"),
    State("dataset", "value"),
    State("columns-x", "value"),
    State("columns-y", "value"),
    prevent_initial_call=True,
)
def update_plot(n_clicks: int | None, asset_key: str | None, x_col: str | None, y_col: str | None):
    if x_col is None or y_col is None or asset_key is None:
        return no_update
    asset = ctx.get_dataset_client(asset_key)
    data = asset.load_columns(columns=[x_col, y_col]).to_pandas()
    return px.scatter(data, x=x_col, y=y_col, template=theme_light)
