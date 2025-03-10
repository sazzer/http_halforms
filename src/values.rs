/// Enumeration of HTTP Methods that can be used in tempaltes.
#[derive(Debug, derive_more::Display)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

/// Enumeration of standard supported Content Types that can be used in templates.
#[derive(Debug, derive_more::Display)]
pub enum TemplateContentType {
    #[display("application/json")]
    Json,
    #[display("application/x-www-form-urlencoded")]
    Form,
}

/// Enumeration of values for field types.
#[derive(Debug, derive_more::Display)]
pub enum TemplatePropertyType {
    #[display("hidden")]
    Hidden,
    #[display("text")]
    Text,
    #[display("search")]
    Search,
    #[display("tel")]
    Tel,
    #[display("url")]
    Url,
    #[display("email")]
    Email,
    #[display("password")]
    Password,
    #[display("datetime")]
    DateTime,
    #[display("date")]
    Date,
    #[display("month")]
    Month,
    #[display("week")]
    Week,
    #[display("time")]
    Time,
    #[display("datetime-local")]
    DateTimeLocal,
    #[display("number")]
    Number,
    #[display("range")]
    Range,
    #[display("color")]
    Color,
    #[display("checkbox")]
    Checkbox,
    #[display("radio")]
    Radio,
    #[display("file")]
    File,
}

/// Enumeration of values for Link Relations.
/// Derived from the IANA list at <https://www.iana.org/assignments/link-relations/link-relations.xhtml>.
#[derive(Debug, derive_more::Display)]
pub enum LinkRelation {
    #[display("about")]
    About,
    #[display("acl")]
    Acl,
    #[display("alternate")]
    Alternate,
    #[display("amphtml")]
    Amphtml,
    #[display("appendix")]
    Appendix,
    #[display("apple-touch-icon")]
    AppleTouchIcon,
    #[display("apple-touch-startup-image")]
    AppleTouchStartupImage,
    #[display("archives")]
    Archives,
    #[display("author")]
    Author,
    #[display("blocked-by")]
    BlockedBy,
    #[display("bookmark")]
    Bookmark,
    #[display("canonical")]
    Canonical,
    #[display("chapter")]
    Chapter,
    #[display("cite-as")]
    CiteAs,
    #[display("collection")]
    Collection,
    #[display("contents")]
    Contents,
    #[display("convertedFrom")]
    ConvertedFrom,
    #[display("copyright")]
    Copyright,
    #[display("create-form")]
    CreateForm,
    #[display("current")]
    Current,
    #[display("describedby")]
    Describedby,
    #[display("describes")]
    Describes,
    #[display("disclosure")]
    Disclosure,
    #[display("dns-prefetch")]
    DnsPrefetch,
    #[display("duplicate")]
    Duplicate,
    #[display("edit")]
    Edit,
    #[display("edit-form")]
    EditForm,
    #[display("edit-media")]
    EditMedia,
    #[display("enclosure")]
    Enclosure,
    #[display("external")]
    External,
    #[display("first")]
    First,
    #[display("glossary")]
    Glossary,
    #[display("help")]
    Help,
    #[display("hosts")]
    Hosts,
    #[display("hub")]
    Hub,
    #[display("icon")]
    Icon,
    #[display("index")]
    Index,
    #[display("intervalAfter")]
    IntervalAfter,
    #[display("intervalBefore")]
    IntervalBefore,
    #[display("intervalContains")]
    IntervalContains,
    #[display("intervalDisjoint")]
    IntervalDisjoint,
    #[display("intervalDuring")]
    IntervalDuring,
    #[display("intervalEquals")]
    IntervalEquals,
    #[display("intervalFinishedBy")]
    IntervalFinishedBy,
    #[display("intervalFinishes")]
    IntervalFinishes,
    #[display("intervalIn")]
    IntervalIn,
    #[display("intervalMeets")]
    IntervalMeets,
    #[display("intervalMetBy")]
    IntervalMetBy,
    #[display("intervalOverlappedBy")]
    IntervalOverlappedBy,
    #[display("intervalOverlaps")]
    IntervalOverlaps,
    #[display("intervalStartedBy")]
    IntervalStartedBy,
    #[display("intervalStarts")]
    IntervalStarts,
    #[display("item")]
    Item,
    #[display("last")]
    Last,
    #[display("latest-version")]
    LatestVersion,
    #[display("license")]
    License,
    #[display("lrdd")]
    Lrdd,
    #[display("manifest")]
    Manifest,
    #[display("mask-icon")]
    MaskIcon,
    #[display("media-feed")]
    MediaFeed,
    #[display("memento")]
    Memento,
    #[display("micropub")]
    Micropub,
    #[display("modulepreload")]
    Modulepreload,
    #[display("monitor")]
    Monitor,
    #[display("monitor-group")]
    MonitorGroup,
    #[display("next")]
    Next,
    #[display("next-archive")]
    NextArchive,
    #[display("nofollow")]
    Nofollow,
    #[display("noopener")]
    Noopener,
    #[display("noreferrer")]
    Noreferrer,
    #[display("opener")]
    Opener,
    #[display("openid2.local_id")]
    Openid2LocalId,
    #[display("openid2.provider")]
    Openid2Provider,
    #[display("original")]
    Original,
    #[display("P3Pv1")]
    P3Pv1,
    #[display("payment")]
    Payment,
    #[display("pingback")]
    Pingback,
    #[display("preconnect")]
    Preconnect,
    #[display("predecessor-version")]
    PredecessorVersion,
    #[display("prefetch")]
    Prefetch,
    #[display("preload")]
    Preload,
    #[display("prerender")]
    Prerender,
    #[display("prev")]
    Prev,
    #[display("preview")]
    Preview,
    #[display("previous")]
    Previous,
    #[display("prev-archive")]
    PrevArchive,
    #[display("privacy-policy")]
    PrivacyPolicy,
    #[display("profile")]
    Profile,
    #[display("publication")]
    Publication,
    #[display("related")]
    Related,
    #[display("restconf")]
    Restconf,
    #[display("replies")]
    Replies,
    #[display("ruleinput")]
    Ruleinput,
    #[display("search")]
    Search,
    #[display("section")]
    Section,
    #[display("self")]
    SelfLink,
    #[display("service")]
    Service,
    #[display("service-desc")]
    ServiceDesc,
    #[display("service-doc")]
    ServiceDoc,
    #[display("service-meta")]
    ServiceMeta,
    #[display("sponsored")]
    Sponsored,
    #[display("start")]
    Start,
    #[display("status")]
    Status,
    #[display("stylesheet")]
    Stylesheet,
    #[display("subsection")]
    Subsection,
    #[display("successor-version")]
    SuccessorVersion,
    #[display("sunset")]
    Sunset,
    #[display("tag")]
    Tag,
    #[display("terms-of-service")]
    TermsOfService,
    #[display("timegate")]
    Timegate,
    #[display("timemap")]
    Timemap,
    #[display("type")]
    Type,
    #[display("ugc")]
    Ugc,
    #[display("up")]
    Up,
    #[display("version-history")]
    VersionHistory,
    #[display("via")]
    Via,
    #[display("webmention")]
    Webmention,
    #[display("working-copy")]
    WorkingCopy,
    #[display("working-copy-of")]
    WorkingCopyOf,
}
