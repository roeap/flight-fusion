import dash_antd as dadc
import plotly.graph_objects as go
from dash import dcc, register_page  # type: ignore

from app.themes import theme_light

register_page(path="/", title="Explorer", icon="ControlOutlined")  # type: ignore

layout = [
    dadc.Row(
        dadc.PageHeader(
            [
                dadc.Input(),
                dadc.PageHeaderOperation(dadc.Button("Update Plot", type="primary")),
            ],
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
            figure=go.Figure(layout={"template": theme_light}),
            responsive=True,
            style={"height": "calc(100vh - 200px)", "width": "100%"},
        ),
        style={"height": "calc(100vh - 200px)", "width": "100%"},
    ),
]
