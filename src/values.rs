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
    #[display(fmt = "application/json")]
    Json,
    #[display(fmt = "application/x-www-form-urlencoded")]
    Form,
}

/// Enumeration of values for field types.
#[derive(Debug, derive_more::Display)]
pub enum TemplatePropertyType {
    #[display(fmt = "hidden")]
    Hidden,
    #[display(fmt = "text")]
    Text,
    #[display(fmt = "search")]
    Search,
    #[display(fmt = "tel")]
    Tel,
    #[display(fmt = "url")]
    Url,
    #[display(fmt = "email")]
    Email,
    #[display(fmt = "password")]
    Password,
    #[display(fmt = "datetime")]
    DateTime,
    #[display(fmt = "date")]
    Date,
    #[display(fmt = "month")]
    Month,
    #[display(fmt = "week")]
    Week,
    #[display(fmt = "time")]
    Time,
    #[display(fmt = "datetime-local")]
    DateTimeLocal,
    #[display(fmt = "number")]
    Number,
    #[display(fmt = "range")]
    Range,
    #[display(fmt = "color")]
    Color,
    #[display(fmt = "checkbox")]
    Checkbox,
    #[display(fmt = "radio")]
    Radio,
    #[display(fmt = "file")]
    File,
}

/// Enumeration of values for Link Relations.
/// Derived from the IANA list at <https://www.iana.org/assignments/link-relations/link-relations.xhtml>.
#[derive(Debug, derive_more::Display)]
pub enum LinkRelation {
    #[display(fmt = "about")]
    About,
    #[display(fmt = "acl")]
    Acl,
    #[display(fmt = "alternate")]
    Alternate,
    #[display(fmt = "amphtml")]
    Amphtml,
    #[display(fmt = "appendix")]
    Appendix,
    #[display(fmt = "apple-touch-icon")]
    AppleTouchIcon,
    #[display(fmt = "apple-touch-startup-image")]
    AppleTouchStartupImage,
    #[display(fmt = "archives")]
    Archives,
    #[display(fmt = "author")]
    Author,
    #[display(fmt = "blocked-by")]
    BlockedBy,
    #[display(fmt = "bookmark")]
    Bookmark,
    #[display(fmt = "canonical")]
    Canonical,
    #[display(fmt = "chapter")]
    Chapter,
    #[display(fmt = "cite-as")]
    CiteAs,
    #[display(fmt = "collection")]
    Collection,
    #[display(fmt = "contents")]
    Contents,
    #[display(fmt = "convertedFrom")]
    ConvertedFrom,
    #[display(fmt = "copyright")]
    Copyright,
    #[display(fmt = "create-form")]
    CreateForm,
    #[display(fmt = "current")]
    Current,
    #[display(fmt = "describedby")]
    Describedby,
    #[display(fmt = "describes")]
    Describes,
    #[display(fmt = "disclosure")]
    Disclosure,
    #[display(fmt = "dns-prefetch")]
    DnsPrefetch,
    #[display(fmt = "duplicate")]
    Duplicate,
    #[display(fmt = "edit")]
    Edit,
    #[display(fmt = "edit-form")]
    EditForm,
    #[display(fmt = "edit-media")]
    EditMedia,
    #[display(fmt = "enclosure")]
    Enclosure,
    #[display(fmt = "external")]
    External,
    #[display(fmt = "first")]
    First,
    #[display(fmt = "glossary")]
    Glossary,
    #[display(fmt = "help")]
    Help,
    #[display(fmt = "hosts")]
    Hosts,
    #[display(fmt = "hub")]
    Hub,
    #[display(fmt = "icon")]
    Icon,
    #[display(fmt = "index")]
    Index,
    #[display(fmt = "intervalAfter")]
    IntervalAfter,
    #[display(fmt = "intervalBefore")]
    IntervalBefore,
    #[display(fmt = "intervalContains")]
    IntervalContains,
    #[display(fmt = "intervalDisjoint")]
    IntervalDisjoint,
    #[display(fmt = "intervalDuring")]
    IntervalDuring,
    #[display(fmt = "intervalEquals")]
    IntervalEquals,
    #[display(fmt = "intervalFinishedBy")]
    IntervalFinishedBy,
    #[display(fmt = "intervalFinishes")]
    IntervalFinishes,
    #[display(fmt = "intervalIn")]
    IntervalIn,
    #[display(fmt = "intervalMeets")]
    IntervalMeets,
    #[display(fmt = "intervalMetBy")]
    IntervalMetBy,
    #[display(fmt = "intervalOverlappedBy")]
    IntervalOverlappedBy,
    #[display(fmt = "intervalOverlaps")]
    IntervalOverlaps,
    #[display(fmt = "intervalStartedBy")]
    IntervalStartedBy,
    #[display(fmt = "intervalStarts")]
    IntervalStarts,
    #[display(fmt = "item")]
    Item,
    #[display(fmt = "last")]
    Last,
    #[display(fmt = "latest-version")]
    LatestVersion,
    #[display(fmt = "license")]
    License,
    #[display(fmt = "lrdd")]
    Lrdd,
    #[display(fmt = "manifest")]
    Manifest,
    #[display(fmt = "mask-icon")]
    MaskIcon,
    #[display(fmt = "media-feed")]
    MediaFeed,
    #[display(fmt = "memento")]
    Memento,
    #[display(fmt = "micropub")]
    Micropub,
    #[display(fmt = "modulepreload")]
    Modulepreload,
    #[display(fmt = "monitor")]
    Monitor,
    #[display(fmt = "monitor-group")]
    MonitorGroup,
    #[display(fmt = "next")]
    Next,
    #[display(fmt = "next-archive")]
    NextArchive,
    #[display(fmt = "nofollow")]
    Nofollow,
    #[display(fmt = "noopener")]
    Noopener,
    #[display(fmt = "noreferrer")]
    Noreferrer,
    #[display(fmt = "opener")]
    Opener,
    #[display(fmt = "openid2.local_id")]
    Openid2LocalId,
    #[display(fmt = "openid2.provider")]
    Openid2Provider,
    #[display(fmt = "original")]
    Original,
    #[display(fmt = "P3Pv1")]
    P3Pv1,
    #[display(fmt = "payment")]
    Payment,
    #[display(fmt = "pingback")]
    Pingback,
    #[display(fmt = "preconnect")]
    Preconnect,
    #[display(fmt = "predecessor-version")]
    PredecessorVersion,
    #[display(fmt = "prefetch")]
    Prefetch,
    #[display(fmt = "preload")]
    Preload,
    #[display(fmt = "prerender")]
    Prerender,
    #[display(fmt = "prev")]
    Prev,
    #[display(fmt = "preview")]
    Preview,
    #[display(fmt = "previous")]
    Previous,
    #[display(fmt = "prev-archive")]
    PrevArchive,
    #[display(fmt = "privacy-policy")]
    PrivacyPolicy,
    #[display(fmt = "profile")]
    Profile,
    #[display(fmt = "publication")]
    Publication,
    #[display(fmt = "related")]
    Related,
    #[display(fmt = "restconf")]
    Restconf,
    #[display(fmt = "replies")]
    Replies,
    #[display(fmt = "ruleinput")]
    Ruleinput,
    #[display(fmt = "search")]
    Search,
    #[display(fmt = "section")]
    Section,
    #[display(fmt = "self")]
    SelfLink,
    #[display(fmt = "service")]
    Service,
    #[display(fmt = "service-desc")]
    ServiceDesc,
    #[display(fmt = "service-doc")]
    ServiceDoc,
    #[display(fmt = "service-meta")]
    ServiceMeta,
    #[display(fmt = "sponsored")]
    Sponsored,
    #[display(fmt = "start")]
    Start,
    #[display(fmt = "status")]
    Status,
    #[display(fmt = "stylesheet")]
    Stylesheet,
    #[display(fmt = "subsection")]
    Subsection,
    #[display(fmt = "successor-version")]
    SuccessorVersion,
    #[display(fmt = "sunset")]
    Sunset,
    #[display(fmt = "tag")]
    Tag,
    #[display(fmt = "terms-of-service")]
    TermsOfService,
    #[display(fmt = "timegate")]
    Timegate,
    #[display(fmt = "timemap")]
    Timemap,
    #[display(fmt = "type")]
    Type,
    #[display(fmt = "ugc")]
    Ugc,
    #[display(fmt = "up")]
    Up,
    #[display(fmt = "version-history")]
    VersionHistory,
    #[display(fmt = "via")]
    Via,
    #[display(fmt = "webmention")]
    Webmention,
    #[display(fmt = "working-copy")]
    WorkingCopy,
    #[display(fmt = "working-copy-of")]
    WorkingCopyOf,
}
