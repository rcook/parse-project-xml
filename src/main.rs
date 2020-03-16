use sxd_document::parser;
use sxd_xpath::{Context, Factory};

struct XmlNamespace {
    prefix: String,
    uri: String,
}

impl XmlNamespace {
    fn new<P: Into<String>, U: Into<String>>(prefix: P, uri: U) -> XmlNamespace {
        XmlNamespace {
            prefix: prefix.into(),
            uri: uri.into(),
        }
    }
}

fn query_xpath_as_string(namespaces: &Vec<XmlNamespace>, query: &str, xml: &str) -> String {
    let package = parser::parse(xml).expect("Failed to parse XML");
    let document = package.as_document();
    let root = document.root().children()[0];

    let mut context = Context::new();
    for namespace in namespaces {
        context.set_namespace(&namespace.prefix, &namespace.uri)
    }

    let factory = Factory::new();
    let xpath = factory
        .build(query)
        .expect("Failed to compile XPath query")
        .expect("No XPath query was compiled");
    xpath
        .evaluate(&context, root)
        .expect("Failed to evaluate XPath query")
        .string()
}

fn get_package_version(project_xml: &str, package_name: &str) -> String {
    query_xpath_as_string(
        &vec![XmlNamespace::new(
            "x",
            "http://schemas.microsoft.com/developer/msbuild/2003",
        )],
        &format!(
            "/x:Project/x:ItemGroup/x:PackageReference[@Include='{}']/x:Version",
            package_name
        ),
        project_xml,
    )
}

fn main() {
    let version = get_package_version(
        "<?xml version=\"1.0\" encoding=\"utf-8\"?>
<Project xmlns=\"http://schemas.microsoft.com/developer/msbuild/2003\">
  <ItemGroup>
    <PackageReference Include=\"PackageA\">
      <Version>Version1</Version>
    </PackageReference>
    <PackageReference Include=\"PackageB\">
      <Version>Version2</Version>
    </PackageReference>
  </ItemGroup>
</Project>
",
        "PackageB",
    );
    assert_eq!(version, "Version2");
    println!("(success)")
}
