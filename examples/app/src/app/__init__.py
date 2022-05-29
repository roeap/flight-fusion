from xml.sax.handler import feature_external_ges

import dash
import dash_antd as dadc

from app.plugin import multi_page

HEADER_HEIGHT = "50px"

app = dash.Dash(__name__, plugins=[multi_page])


def get_nav_item(page):
    if "icon" in page:
        return {
            "label": page["title"],
            "key": page["path"],
            "path": page["path"],
            "icon": page["icon"],
        }
    return {"label": page["title"], "key": page["path"], "path": page["path"]}


nav_items = [get_nav_item(page) for page in dash.page_registry.values()]  # type: ignore


app.layout = dadc.Layout(
    has_sidebar=False,
    style={"height": "100vh", "overflow": "auto"},
    children=[
        dadc.Header(
            dadc.Icon(
                icon_name="BorderRightOutlined",
                style={"color": "red", "fontSize": 50, "marginLeft": -50},
            ),
            style={"background": "#004A96", "height": HEADER_HEIGHT},
        ),
        dadc.Layout(
            children=[
                dadc.Sidebar(
                    style={
                        "overflow": "auto",
                        "borderRight": "1px solid rgba(0,0,0,.06)",
                    },
                    theme="light",
                    children=[
                        dadc.Menu(
                            id="page-nav", items=nav_items, selected_keys=["page-1"]
                        ),
                        dadc.Divider("Controls"),
                    ],
                ),
                dadc.Content(
                    multi_page.page_container,
                    id="page-content",
                ),
            ],
        ),
    ],
)
