use std::rc::Rc;
use syntax::ext::base::ExtCtxt;

use builder::{Builder, TokenString, add_node_dependency};
use node;

pub fn attach(builder: &mut Builder, _: &mut ExtCtxt, node: Rc<node::Node>) {
  node.materializer.set(Some(verify));
  for port_node in node.subnodes().iter() {
    port_node.materializer.set(Some(verify));
    add_node_dependency(&node, port_node);
    for pin_node in port_node.subnodes().iter() {
      pin_node.materializer.set(Some(build_pin));
      add_node_dependency(port_node, pin_node);
      super::add_node_dependency_on_clock(builder, pin_node);
    }
  }
}

pub fn verify(_: &mut Builder, cx: &mut ExtCtxt, node: Rc<node::Node>) {
  node.expect_no_attributes(cx);
}

fn build_pin(builder: &mut Builder, cx: &mut ExtCtxt, node: Rc<node::Node>) {
  let port_node = node.parent.clone().unwrap().upgrade().unwrap();
  let ref port_path = port_node.path;
  let port = TokenString(port_path.clone());

  let error = | err: &str | {
    cx.parse_sess().span_diagnostic.span_err(port_node.path_span, err);
  };


  if node.name.is_none() {
    error("pin node must have a name");
    return;
  }

  let direction_str =
    match node.get_string_attr("direction").unwrap().as_slice() {
      "out" => "zinc::hal::pin::Out",
      "in"  => "zinc::hal::pin::In",
      bad   => {
        error(format!("unknown direction `{}`, allowed values: `in`, `out`",
                      bad).as_slice());
        return;
      }
    };

  let direction = TokenString(direction_str.to_string());

  let pin_str = match from_str::<uint>(node.path.as_slice()).unwrap() {
    0 ...7  => &node.path,
    other  => {
      error(format!("unknown pin `{}`, allowed values: 0...7",
                    other).as_slice());
      return;
    }
  };

  let pin = TokenString(format!("{}u8", pin_str));
  let pin_name = TokenString(node.name.clone().unwrap());

  node.set_type_name("zinc::hal::tiva_c::pin::Pin".to_string());

  /* XXX: need to handle pin muxing */
  let st = quote_stmt!(&*cx,
      let $pin_name = zinc::hal::tiva_c::pin::Pin::new(
          zinc::hal::tiva_c::pin::$port,
          $pin,
          $direction);
  );
  builder.add_main_statement(st);
}
