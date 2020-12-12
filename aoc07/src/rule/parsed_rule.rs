use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, space1},
    multi::separated_list1,
    sequence::{terminated, tuple},
    IResult,
};

type ContainedBag = (String, u32);

pub struct ParsedRule {
    pub parent: String,
    pub children: Vec<ContainedBag>,
}

impl From<&str> for ParsedRule {
    fn from(line: &str) -> Self {
        let (_, (parent, _, children)) = tuple((bag_name, contains, contained_bags))(line).unwrap();

        Self { parent, children }
    }
}

fn bag_name(input: &str) -> IResult<&str, String> {
    let (rem, name_parts) = tuple((alpha1, space1, alpha1))(input)?;

    Ok((rem, name_parts.0.to_owned() + name_parts.1 + name_parts.2))
}

fn contains(input: &str) -> IResult<&str, ()> {
    let (rem, _) = tag(" bags contain ")(input)?;

    Ok((rem, ()))
}

fn contained_bags(input: &str) -> IResult<&str, Vec<ContainedBag>> {
    let (rem, children) = terminated(alt((empty_bag, child_bags)), tag("."))(input)?;

    Ok((rem, children.unwrap_or(vec![])))
}

fn child_bags(input: &str) -> IResult<&str, Option<Vec<ContainedBag>>> {
    let (rem, bag_list) = separated_list1(tag(", "), child_bag)(input)?;

    Ok((rem, Some(bag_list)))
}

fn child_bag(input: &str) -> IResult<&str, ContainedBag> {
    let (rem, x) = tuple((
        digit1,
        space1,
        bag_name,
        space1,
        alt((tag("bags"), tag("bag"))),
    ))(input)?;

    Ok((rem, (x.2, x.0.parse().unwrap())))
}

fn empty_bag(input: &str) -> IResult<&str, Option<Vec<ContainedBag>>> {
    let (rem, _) = tag("no other bags")(input)?;

    Ok((rem, None))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bag_name() {
        assert_eq!(
            bag_name("drab plum bags contain 5 clear turquoise bags."),
            Ok((
                " bags contain 5 clear turquoise bags.",
                String::from("drab plum")
            ))
        );
    }

    #[test]
    fn test_contains() {
        assert_eq!(
            contains(" bags contain 5 clear turquoise bags."),
            Ok(("5 clear turquoise bags.", ()))
        );
    }

    #[test]
    fn test_empty_bag() {
        assert_eq!(empty_bag("no other bags"), Ok(("", None)));
    }

    #[test]
    fn test_child_bag() {
        assert_eq!(
            child_bag("5 clear turquoise bags"),
            Ok(("", ("clear turquoise".into(), 5)))
        );

        assert_eq!(
            child_bag("1 clear turquoise bag"),
            Ok(("", ("clear turquoise".into(), 1)))
        );
    }

    #[test]
    fn test_child_bags() {
        assert_eq!(
            child_bags("5 clear turquoise bags, 1 drab plum bag"),
            Ok((
                "",
                Some(vec![("clear turquoise".into(), 5), ("drab plum".into(), 1)])
            ))
        );
    }

    #[test]
    fn test_contained_bags() {
        assert_eq!(
            contained_bags("5 clear turquoise bags, 1 drab plum bag.").unwrap(),
            (
                "",
                vec![("clear turquoise".into(), 5), ("drab plum".into(), 1)]
            )
        );

        assert_eq!(contained_bags("no other bags.").unwrap(), ("", vec![]));
    }
}
