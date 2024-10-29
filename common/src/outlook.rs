use serde::Serialize;

/**
 * Generated using python script `tools/Outlook/generate_properties.py`
 */
#[derive(Debug, PartialEq, Eq, Serialize, Clone)]
pub enum PropertyName {
    PidTag7BitDisplayName,
    PidTagAccess,
    PidTagAccessControlListData,
    PidTagAccessLevel,
    PidTagAccount,
    PidTagAdditionalRenEntryIds,
    PidTagAdditionalRenEntryIdsEx,
    PidTagAddressBookAuthorizedSenders,
    PidTagAddressBookContainerId,
    PidTagAddressBookDeliveryContentLength,
    PidTagAddressBookDisplayNamePrintable,
    PidTagAddressBookDisplayTypeExtended,
    PidTagAddressBookDistributionListExternalMemberCount,
    PidTagAddressBookDistributionListMemberCount,
    PidTagAddressBookDistributionListMemberSubmitAccepted,
    PidTagAddressBookDistributionListMemberSubmitRejected,
    PidTagAddressBookDistributionListRejectMessagesFromDLMembers,
    PidTagAddressBookEntryId,
    PidTagAddressBookExtensionAttribute1,
    PidTagAddressBookExtensionAttribute10,
    PidTagAddressBookExtensionAttribute11,
    PidTagAddressBookExtensionAttribute12,
    PidTagAddressBookExtensionAttribute13,
    PidTagAddressBookExtensionAttribute14,
    PidTagAddressBookExtensionAttribute15,
    PidTagAddressBookExtensionAttribute2,
    PidTagAddressBookExtensionAttribute3,
    PidTagAddressBookExtensionAttribute4,
    PidTagAddressBookExtensionAttribute5,
    PidTagAddressBookExtensionAttribute6,
    PidTagAddressBookExtensionAttribute7,
    PidTagAddressBookExtensionAttribute8,
    PidTagAddressBookExtensionAttribute9,
    PidTagAddressBookFolderPathname,
    PidTagAddressBookHierarchicalChildDepartments,
    PidTagAddressBookHierarchicalDepartmentMembers,
    PidTagAddressBookHierarchicalIsHierarchicalGroup,
    PidTagAddressBookHierarchicalParentDepartment,
    PidTagAddressBookHierarchicalRootDepartment,
    PidTagAddressBookHierarchicalShowInDepartments,
    PidTagAddressBookHomeMessageDatabase,
    PidTagAddressBookIsMaster,
    PidTagAddressBookIsMemberOfDistributionList,
    PidTagAddressBookIsMemberOfDistributionListNspi,
    PidTagAddressBookManageDistributionList,
    PidTagAddressBookManager,
    PidTagAddressBookManagerDistinguishedName,
    PidTagAddressBookMember,
    PidTagAddressBookMessageId,
    PidTagAddressBookModerationEnabled,
    PidTagAddressBookNetworkAddress,
    PidTagAddressBookObjectDistinguishedName,
    PidTagAddressBookObjectGuid,
    PidTagAddressBookOrganizationalUnitRootDistinguishedName,
    PidTagAddressBookOwner,
    PidTagAddressBookOwnerBackLink,
    PidTagAddressBookParentEntryId,
    PidTagAddressBookPhoneticCompanyName,
    PidTagAddressBookPhoneticDepartmentName,
    PidTagAddressBookPhoneticDisplayName,
    PidTagAddressBookPhoneticGivenName,
    PidTagAddressBookPhoneticSurname,
    PidTagAddressBookProxyAddresses,
    PidTagAddressBookPublicDelegates,
    PidTagAddressBookReports,
    PidTagAddressBookRoomCapacity,
    PidTagAddressBookRoomContainers,
    PidTagAddressBookRoomDescription,
    PidTagAddressBookSenderHintTranslations,
    PidTagAddressBookSeniorityIndex,
    PidTagAddressBookTargetAddress,
    PidTagAddressBookUnauthorizedSenders,
    PidTagAddressBookX509Certificate,
    PidTagAddressType,
    PidTagAlternateRecipientAllowed,
    PidTagAnr,
    PidTagArchiveDate,
    PidTagArchivePeriod,
    PidTagArchiveTag,
    PidTagAssistant,
    PidTagAssistantTelephoneNumber,
    PidTagAssistantTelephoneNumberW,
    PidTagAssociated,
    PidTagAttachAdditionalInformation,
    PidTagAttachContentBase,
    PidTagAttachContentId,
    PidTagAttachContentLocation,
    PidTagAttachDataBinary,
    PidTagAttachDataObject,
    PidTagAttachEncoding,
    PidTagAttachExtension,
    PidTagAttachExtensionW,
    PidTagAttachFilename,
    PidTagAttachFilenameW,
    PidTagAttachFlags,
    PidTagAttachLongFilename,
    PidTagAttachLongFilenameW,
    PidTagAttachLongPathname,
    PidTagAttachLongPathnameW,
    PidTagAttachmentContactPhoto,
    PidTagAttachmentFlags,
    PidTagAttachmentHidden,
    PidTagAttachmentLinkId,
    PidTagAttachMethod,
    PidTagAttachMimeSequence,
    PidTagAttachMimeTag,
    PidTagAttachMimeTagW,
    PidTagAttachNumber,
    PidTagAttachPathname,
    PidTagAttachPathnameW,
    PidTagAttachPayloadClass,
    PidTagAttachPayloadProviderGuidString,
    PidTagAttachRendering,
    PidTagAttachSize,
    PidTagAttachTag,
    PidTagAttachTransportName,
    PidTagAttributeHidden,
    PidTagAttributeReadOnly,
    PidTagAttributeSystem,
    PidTagAutoForwardComment,
    PidTagAutoForwarded,
    PidTagAutoResponseSuppress,
    PidTagBirthday,
    PidTagBlockStatus,
    PidTagBody,
    PidTagBodyContentId,
    PidTagBodyContentLocation,
    PidTagBodyHtml,
    PidTagBodyHtmlW,
    PidTagBodyW,
    PidTagBusiness2TelephoneNumber,
    PidTagBusiness2TelephoneNumbers,
    PidTagBusiness2TelephoneNumberW,
    PidTagBusinessFaxNumber,
    PidTagBusinessFaxNumberW,
    PidTagBusinessHomePage,
    PidTagBusinessTelephoneNumber,
    PidTagBusinessTelephoneNumberW,
    PidTagCalendarType,
    PidTagCallbackTelephoneNumber,
    PidTagCallId,
    PidTagCarTelephoneNumber,
    PidTagCarTelephoneNumberW,
    PidTagCdoRecurrenceid,
    PidTagChangeKey,
    PidTagChangeNotificationGuid,
    PidTagChangeNumber,
    PidTagChildrensNames,
    PidTagClientActions,
    PidTagClientSubmitTime,
    PidTagCnsetRead,
    PidTagCnsetSeen,
    PidTagCnsetSeenFAI,
    PidTagCodePageId,
    PidTagComment,
    PidTagCommentW,
    PidTagCommonViewsEntryId,
    PidTagCompanyMainTelephoneNumber,
    PidTagCompanyMainTelephoneNumberW,
    PidTagCompanyName,
    PidTagCompanyNameW,
    PidTagComputerNetworkName,
    PidTagConflictEntryId,
    PidTagConflictItems,
    PidTagContainerClass,
    PidTagContainerClassW,
    PidTagContainerContents,
    PidTagContainerFlags,
    PidTagContainerHierarchy,
    PidTagContentCount,
    PidTagContentFilterPhishingConfidenceLevel,
    PidTagContentFilterSpamConfidenceLevel,
    PidTagContentUnreadCount,
    PidTagConversationId,
    PidTagConversationIndex,
    PidTagConversationIndexTracking,
    PidTagConversationKey,
    PidTagConversationTopic,
    PidTagConversationTopicW,
    PidTagConvertHtmlSecurity,
    PidTagCorrelateMtsid,
    PidTagCountry,
    PidTagCreateTemplates,
    PidTagCreationTime,
    PidTagCreationVersion,
    PidTagCreatorAddressType,
    PidTagCreatorEmailAddress,
    PidTagCreatorEntryId,
    PidTagCreatorName,
    PidTagCreatorSimpleDisplayName,
    PidTagCustomerId,
    PidTagDamBackPatched,
    PidTagDamOriginalEntryId,
    PidTagDayInterval,
    PidTagDefaultPostMessageClass,
    PidTagDeferredActionMessageOriginalEntryId,
    PidTagDeferredDeliveryTime,
    PidTagDeferredSendNumber,
    PidTagDeferredSendTime,
    PidTagDeferredSendUnits,
    PidTagDelegatedByRule,
    PidTagDelegateFlags,
    PidTagDelegateMail,
    PidTagDelegation,
    PidTagDeleteAfterSubmit,
    PidTagDeletedAssociatedMessageCount,
    PidTagDeletedAssociatedMessageSizeExtended,
    PidTagDeletedCountTotal,
    PidTagDeletedMessageCount,
    PidTagDeletedMessageSizeExtended,
    PidTagDeletedNormalMessageSizeExtended,
    PidTagDeletedOn,
    PidTagDeliverTime,
    PidTagDepartmentName,
    PidTagDepth,
    PidTagDisplayBcc,
    PidTagDisplayBccW,
    PidTagDisplayCc,
    PidTagDisplayCcW,
    PidTagDisplayName,
    PidTagDisplayNamePrefix,
    PidTagDisplayNameW,
    PidTagDisplayTo,
    PidTagDisplayToW,
    PidTagDisplayType,
    PidTagDisplayTypeEx,
    PidTagEcWarning,
    PidTagEmailAddress,
    PidTagEmailAddress1,
    PidTagEmailAddressW,
    PidTagEndAttach,
    PidTagEndDate,
    PidTagEndEmbed,
    PidTagEndFolder,
    PidTagEndMessage,
    PidTagEndRecurrenceTime,
    PidTagEndToRecip,
    PidTagEntryId,
    PidTagExceptionEndTime,
    PidTagExceptionReplaceTime,
    PidTagExceptionStartTime,
    PidTagExchangeNTSecurityDescriptor,
    PidTagExpiryNumber,
    PidTagExpiryTime,
    PidTagExpiryUnits,
    PidTagExtendedFolderFlags,
    PidTagExtendedRuleMessageActions,
    PidTagExtendedRuleMessageCondition,
    PidTagExtendedRuleSizeLimit,
    PidTagFaxNumberOfPages,
    PidTagFinderEntryId,
    PidTagFlagCompleteTime,
    PidTagFlagStatus,
    PidTagFlatUrlName,
    PidTagFolderAssociatedContents,
    PidTagFolderFlags,
    PidTagFolderId,
    PidTagFolderId32,
    PidTagFolderType,
    PidTagFollowupIcon,
    PidTagFreeBusyCountMonths,
    PidTagFreeBusyEntryIds,
    PidTagFreeBusyMessageEmailAddress,
    PidTagFreeBusyPublishEnd,
    PidTagFreeBusyPublishStart,
    PidTagFreeBusyRangeTimestamp,
    PidTagFtpSite,
    PidTagFXDelProp,
    PidTagFXErrorInfo,
    PidTagGatewayNeedsToRefresh,
    PidTagGender,
    PidTagGeneration,
    PidTagGivenName,
    PidTagGivenNameW,
    PidTagGovernmentIdNumber,
    PidTagHasAttachments,
    PidTagHasDeferredActionMessages,
    PidTagHasNamedProperties,
    PidTagHasRules,
    PidTagHierarchyChangeNumber,
    PidTagHierRev,
    PidTagHobbies,
    PidTagHome2TelephoneNumber,
    PidTagHome2TelephoneNumbers,
    PidTagHome2TelephoneNumberW,
    PidTagHomeAddressCity,
    PidTagHomeAddressCountry,
    PidTagHomeAddressPostalCode,
    PidTagHomeAddressPostOfficeBox,
    PidTagHomeAddressStateOrProvince,
    PidTagHomeAddressStreet,
    PidTagHomeFaxNumber,
    PidTagHomeFaxNumberW,
    PidTagHomeTelephoneNumber,
    PidTagHomeTelephoneNumberW,
    PidTagHtml,
    PidTagICalendarEndTime,
    PidTagICalendarReminderNextTime,
    PidTagICalendarStartTime,
    PidTagIconIndex,
    PidTagIdsetDeleted,
    PidTagIdsetExpired,
    PidTagIdsetGiven,
    PidTagIdsetNoLongerInScope,
    PidTagIdsetRead,
    PidTagIdsetSoftDeleted,
    PidTagIdsetUnread,
    PidTagImapCachedMsgsize,
    PidTagImportance,
    PidTagInConflict,
    PidTagIncrementalSyncMessagePartial,
    PidTagIncrSyncChg,
    PidTagIncrSyncChgPartial,
    PidTagIncrSyncDel,
    PidTagIncrSyncEnd,
    PidTagIncrSyncGroupId,
    PidTagIncrSyncGroupInfo,
    PidTagIncrSyncMessage,
    PidTagIncrSyncProgressMode,
    PidTagIncrSyncProgressPerMsg,
    PidTagIncrSyncRead,
    PidTagIncrSyncStateBegin,
    PidTagIncrSyncStateEnd,
    PidTagInitialDetailsPane,
    PidTagInitials,
    PidTagInReplyToId,
    PidTagInReplyToIdW,
    PidTagInstanceKey,
    PidTagInstanceNum,
    PidTagInstID,
    PidTagInternetArticleNumber,
    PidTagInternetCodepage,
    PidTagInternetLines,
    PidTagInternetMailOverrideFormat,
    PidTagInternetMessageId,
    PidTagInternetMessageIdW,
    PidTagInternetOrganization,
    PidTagInternetReferences,
    PidTagInternetReturnPath,
    PidTagIpmAppointmentEntryId,
    PidTagIpmContactEntryId,
    PidTagIpmDraftsEntryId,
    PidTagIpmJournalEntryId,
    PidTagIpmNoteEntryId,
    PidTagIpmOutboxEntryId,
    PidTagIpmSentMailEntryId,
    PidTagIpmSubTreeEntryId,
    PidTagIpmTaskEntryId,
    PidTagIpmWasteBasketEntryId,
    PidTagIsdnNumber,
    PidTagIsdnNumberW,
    PidTagIsException,
    PidTagItemTemporaryFlags,
    PidTagJunkAddRecipientsToSafeSendersList,
    PidTagJunkIncludeContacts,
    PidTagJunkPermanentlyDelete,
    PidTagJunkPhishingEnableLinks,
    PidTagJunkPostmarkOutgoingMail,
    PidTagJunkThreshold,
    PidTagKeyword,
    PidTagLanguage,
    PidTagLastModificationTime,
    PidTagLastModifierEntryId,
    PidTagLastModifierName,
    PidTagLastModifierSimpleDisplayName,
    PidTagLastVerbExecuted,
    PidTagLastVerbExecutionTime,
    PidTagListHelp,
    PidTagListSubscribe,
    PidTagListUnsubscribe,
    PidTagLocalCommitTime,
    PidTagLocalCommitTimeMax,
    PidTagLocaleId,
    PidTagLocality,
    PidTagLocation,
    PidTagLtpParentNid,
    PidTagLtpPstPassword,
    PidTagLtpRowId,
    PidTagLtpRowVer,
    PidTagMailboxOwnerEntryId,
    PidTagMailboxOwnerName,
    PidTagManagerName,
    PidTagMapiFormComposeCommand,
    PidTagMappingSignature,
    PidTagMaximumSubmitMessageSize,
    PidTagMemberId,
    PidTagMemberName,
    PidTagMemberRights,
    PidTagMessageAttachments,
    PidTagMessageCcMe,
    PidTagMessageClass,
    PidTagMessageClassW,
    PidTagMessageCodepage,
    PidTagMessageDeliveryTime,
    PidTagMessageEditorFormat,
    PidTagMessageFlags,
    PidTagMessageHandlingSystemCommonName,
    PidTagMessageLocaleId,
    PidTagMessageRecipientMe,
    PidTagMessageRecipients,
    PidTagMessageSize,
    PidTagMessageSizeExtended,
    PidTagMessageStatus,
    PidTagMessageSubmissionId,
    PidTagMessageToMe,
    PidTagMid,
    PidTagMiddleName,
    PidTagMiddleNameW,
    PidTagMimeSkeleton,
    PidTagMobileTelephoneNumber,
    PidTagMobileTelephoneNumberW,
    PidTagMonthInterval,
    PidTagMsgFolderTemplateRes13,
    PidTagNativeBody,
    PidTagNewAttach,
    PidTagNewFXFolder,
    PidTagNextSendAcct,
    PidTagNextSendAcctW,
    PidTagNickname,
    PidTagNonDeliveryReportDiagCode,
    PidTagNonDeliveryReportReasonCode,
    PidTagNonDeliveryReportStatusCode,
    PidTagNonReceiptNotificationRequested,
    PidTagNormalizedSubject,
    PidTagNormalMessageSize,
    PidTagObjectType,
    PidTagOfficeLocation,
    PidTagOfflineAddressBookCompressedSize,
    PidTagOfflineAddressBookContainerGuid,
    PidTagOfflineAddressBookDistinguishedName,
    PidTagOfflineAddressBookFileSize,
    PidTagOfflineAddressBookFileType,
    PidTagOfflineAddressBookLanguageId,
    PidTagOfflineAddressBookMessageClass,
    PidTagOfflineAddressBookName,
    PidTagOfflineAddressBookSequence,
    PidTagOfflineAddressBookShaHash,
    PidTagOfflineAddressBookTruncatedProperties,
    PidTagOldLocation,
    PidTagOldRecurrenceType,
    PidTagOptionalAttendees,
    PidTagOrdinalMost,
    PidTagOrganizationalIdNumber,
    PidTagOriginalAuthorEntryId,
    PidTagOriginalAuthorName,
    PidTagOriginalDeliveryTime,
    PidTagOriginalDisplayBcc,
    PidTagOriginalDisplayCc,
    PidTagOriginalDisplayName,
    PidTagOriginalDisplayTo,
    PidTagOriginalEntryId,
    PidTagOriginalMessageClass,
    PidTagOriginalMessageEntryId,
    PidTagOriginalMessageId,
    PidTagOriginalMessageIdW,
    PidTagOriginalSearchKey,
    PidTagOriginalSenderAddressType,
    PidTagOriginalSenderEmailAddress,
    PidTagOriginalSenderEntryId,
    PidTagOriginalSenderName,
    PidTagOriginalSenderSearchKey,
    PidTagOriginalSensitivity,
    PidTagOriginalSentRepresentingAddressType,
    PidTagOriginalSentRepresentingEmailAddress,
    PidTagOriginalSentRepresentingEntryId,
    PidTagOriginalSentRepresentingName,
    PidTagOriginalSentRepresentingSearchKey,
    PidTagOriginalSubject,
    PidTagOriginalSubmitTime,
    PidTagOriginatorDeliveryReportRequested,
    PidTagOriginatorNonDeliveryReportRequested,
    PidTagOscSyncEnabled,
    PidTagOtherAddressCity,
    PidTagOtherAddressCountry,
    PidTagOtherAddressPostalCode,
    PidTagOtherAddressPostOfficeBox,
    PidTagOtherAddressStateOrProvince,
    PidTagOtherAddressStreet,
    PidTagOtherTelephoneNumber,
    PidTagOtherTelephoneNumberW,
    PidTagOutOfOfficeState,
    PidTagOwnerAppointmentId,
    PidTagPagerTelephoneNumber,
    PidTagPagerTelephoneNumberW,
    PidTagParentDisplayW,
    PidTagParentEntryId,
    PidTagParentFolderId,
    PidTagParentKey,
    PidTagParentSourceKey,
    PidTagPersonalHomePage,
    PidTagPolicyTag,
    PidTagPostalAddress,
    PidTagPostalAddressW,
    PidTagPostalCode,
    PidTagPostOfficeBox,
    PidTagPredecessorChangeList,
    PidTagPreferredByName,
    PidTagPrimaryFaxNumber,
    PidTagPrimarySendAccount,
    PidTagPrimarySendAccountW,
    PidTagPrimarySmtpAddress,
    PidTagPrimaryTelephoneNumber,
    PidTagPriority,
    PidTagProcessed,
    PidTagProfession,
    PidTagProhibitReceiveQuota,
    PidTagProhibitSendQuota,
    PidTagProviderSubmitTime,
    PidTagPstBodyPrefix,
    PidTagPstHiddenCount,
    PidTagPstHiddenUnread,
    PidTagPstLrNoRestrictions,
    PidTagPublicFolderAdministrativeDescription,
    PidTagPublicFolderProxy,
    PidTagPurportedSenderDomain,
    PidTagRadioTelephoneNumber,
    PidTagRadioTelephoneNumberW,
    PidTagRead,
    PidTagReadReceiptAddressType,
    PidTagReadReceiptEmailAddress,
    PidTagReadReceiptEntryId,
    PidTagReadReceiptName,
    PidTagReadReceiptRequested,
    PidTagReadReceiptSearchKey,
    PidTagReadReceiptSmtpAddress,
    PidTagReceiptTime,
    PidTagReceivedByAddressType,
    PidTagReceivedByAddressTypeW,
    PidTagReceivedByEmailAddress,
    PidTagReceivedByEmailAddressW,
    PidTagReceivedByEntryId,
    PidTagReceivedByFlags,
    PidTagReceivedByName,
    PidTagReceivedByNameW,
    PidTagReceivedBySearchKey,
    PidTagReceivedBySmtpAddress,
    PidTagReceivedRepresentingAddressType,
    PidTagReceivedRepresentingEmailAddress,
    PidTagReceivedRepresentingEntryId,
    PidTagReceivedRepresentingFlags,
    PidTagReceivedRepresentingName,
    PidTagReceivedRepresentingSearchKey,
    PidTagReceivedRepresentingSimpleDisplayName,
    PidTagReceivedRepresentingSmtpAddress,
    PidTagRecipientDisplayName,
    PidTagRecipientEntryId,
    PidTagRecipientFlags,
    PidTagRecipientOrder,
    PidTagRecipientProposed,
    PidTagRecipientProposedEndTime,
    PidTagRecipientProposedStartTime,
    PidTagRecipientReassignmentProhibited,
    PidTagRecipientResourceState,
    PidTagRecipientTrackStatus,
    PidTagRecipientTrackStatusTime,
    PidTagRecipientType,
    PidTagRecordKey,
    PidTagReferredByName,
    PidTagRemindersOnlineEntryId,
    PidTagRemoteHeaderLoc,
    PidTagRemoteMessageTransferAgent,
    PidTagRenderingPosition,
    PidTagReplyRecipientEntries,
    PidTagReplyRecipientNames,
    PidTagReplyRecipientNamesW,
    PidTagReplyRequested,
    PidTagReplyTemplateId,
    PidTagReplyTime,
    PidTagReportDisposition,
    PidTagReportDispositionMode,
    PidTagReportDispositionModeW,
    PidTagReportDispositionToEmailAddresses,
    PidTagReportDispositionToNames,
    PidTagReportDispositionW,
    PidTagReportEntryId,
    PidTagReportingMessageTransferAgent,
    PidTagReportName,
    PidTagReportOriginalSender,
    PidTagReportSearchKey,
    PidTagReportTag,
    PidTagReportText,
    PidTagReportTime,
    PidTagRequiredAttendees,
    PidTagResolveMethod,
    PidTagResourceAttendees,
    PidTagResponseRequested,
    PidTagResponsibility,
    PidTagRetentionDate,
    PidTagRetentionFlags,
    PidTagRetentionPeriod,
    PidTagRights,
    PidTagRoamingDatatypes,
    PidTagRoamingDictionary,
    PidTagRoamingXmlStream,
    PidTagRowid,
    PidTagRowType,
    PidTagRtfCompressed,
    PidTagRtfInSync,
    PidTagRtfSyncBodyCount,
    PidTagRtfSyncBodyCrc,
    PidTagRtfSyncBodyTag,
    PidTagRtfSyncBodyTagW,
    PidTagRtfSyncPrefixCount,
    PidTagRtfSyncTrailingCount,
    PidTagRuleActionNumber,
    PidTagRuleActions,
    PidTagRuleActionType,
    PidTagRuleCondition,
    PidTagRuleError,
    PidTagRuleFolderEntryId,
    PidTagRuleId,
    PidTagRuleIds,
    PidTagRuleLevel,
    PidTagRuleMessageLevel,
    PidTagRuleMessageName,
    PidTagRuleMessageProvider,
    PidTagRuleMessageProviderData,
    PidTagRuleMessageSequence,
    PidTagRuleMessageState,
    PidTagRuleMessageUserFlags,
    PidTagRuleMsgActions,
    PidTagRuleMsgCondition,
    PidTagRuleMsgLevel,
    PidTagRuleMsgName,
    PidTagRuleMsgProvider,
    PidTagRuleMsgProviderData,
    PidTagRuleMsgSequence,
    PidTagRuleMsgState,
    PidTagRuleMsgUserFlags,
    PidTagRuleName,
    PidTagRuleProvider,
    PidTagRuleProviderData,
    PidTagRuleSequence,
    PidTagRuleState,
    PidTagRuleUserFlags,
    PidTagRwRulesStream,
    PidTagScheduleInfoAppointmentTombstone,
    PidTagScheduleInfoAutoAcceptAppointments,
    PidTagScheduleInfoDelegateEntryIds,
    PidTagScheduleInfoDelegateNames,
    PidTagScheduleInfoDelegateNamesW,
    PidTagScheduleInfoDelegatorWantsCopy,
    PidTagScheduleInfoDelegatorWantsInfo,
    PidTagScheduleInfoDisallowOverlappingAppts,
    PidTagScheduleInfoDisallowRecurringAppts,
    PidTagScheduleInfoDontMailDelegates,
    PidTagScheduleInfoFreeBusy,
    PidTagScheduleInfoFreeBusyAway,
    PidTagScheduleInfoFreeBusyBusy,
    PidTagScheduleInfoFreeBusyMerged,
    PidTagScheduleInfoFreeBusyTentative,
    PidTagScheduleInfoMonthsAway,
    PidTagScheduleInfoMonthsBusy,
    PidTagScheduleInfoMonthsMerged,
    PidTagScheduleInfoMonthsTentative,
    PidTagScheduleInfoResourceType,
    PidTagSchedulePlusFreeBusyEntryId,
    PidTagScriptData,
    PidTagSearchFolderDefinition,
    PidTagSearchFolderDefinitions,
    PidTagSearchFolderEfpFlags,
    PidTagSearchFolderExpiration,
    PidTagSearchFolderId,
    PidTagSearchFolderLastUsed,
    PidTagSearchFolderRecreateInfo,
    PidTagSearchFolderStorageType,
    PidTagSearchFolderTag,
    PidTagSearchFolderTemplateId,
    PidTagSearchKey,
    PidTagSecureSubmitFlags,
    PidTagSecurityDescriptor,
    PidTagSecurityDescriptorAsXml,
    PidTagSelectable,
    PidTagSenderAddressType,
    PidTagSenderAddressTypeW,
    PidTagSenderEmailAddress,
    PidTagSenderEmailAddressW,
    PidTagSenderEntryId,
    PidTagSenderFlags,
    PidTagSenderIdStatus,
    PidTagSenderName,
    PidTagSenderNameW,
    PidTagSenderSearchKey,
    PidTagSenderSimpleDisplayName,
    PidTagSenderSmtpAddress,
    PidTagSenderTelephoneNumber,
    PidTagSendInternetEncoding,
    PidTagSendOutlookRecallReport,
    PidTagSendRichInfo,
    PidTagSensitivity,
    PidTagSentMailEntryId,
    PidTagSentMailSvrEID,
    PidTagSentRepresentingAddressType,
    PidTagSentRepresentingAddressTypeW,
    PidTagSentRepresentingEmailAddress,
    PidTagSentRepresentingEmailAddressW,
    PidTagSentRepresentingEntryId,
    PidTagSentRepresentingFlags,
    PidTagSentRepresentingName,
    PidTagSentRepresentingNameW,
    PidTagSentRepresentingSearchKey,
    PidTagSentRepresentingSimpleDisplayName,
    PidTagSentRepresentingSmtpAddress,
    PidTagSerializedReplidGuidMap,
    PidTagSessionInitiationProtocolUri,
    PidTagSmtpAddress,
    PidTagSortLocaleId,
    PidTagSourceKey,
    PidTagSpokenName,
    PidTagSpouseName,
    PidTagStartDate,
    PidTagStartDateEtc,
    PidTagStartEmbed,
    PidTagStartFAIMsg,
    PidTagStartMessage,
    PidTagStartRecip,
    PidTagStartRecurrenceDate,
    PidTagStartRecurrenceTime,
    PidTagStartSubFld,
    PidTagStartTopFld,
    PidTagStateOrProvince,
    PidTagStatus,
    PidTagStoreEntryId,
    PidTagStoreState,
    PidTagStoreSupportMask,
    PidTagStoreUserEntryId,
    PidTagStreetAddress,
    PidTagSubfolder,
    PidTagSubfolders,
    PidTagSubject,
    PidTagSubjectPrefix,
    PidTagSubjectW,
    PidTagSubmitFlags,
    PidTagSupplementaryInfo,
    PidTagSurname,
    PidTagSurnameW,
    PidTagSwappedToDoData,
    PidTagSwappedToDoStore,
    PidTagTargetEntryId,
    PidTagTcvConstLongOne,
    PidTagTelecommunicationsDeviceForDeafTelephoneNumber,
    PidTagTelecommunicationsDeviceForDeafTelephoneNumberW,
    PidTagTelexNumber,
    PidTagTelexNumberNspi,
    PidTagTelexNumberW,
    PidTagTemplateData,
    PidTagTemplateid,
    PidTagTemporaryDefaultDocument,
    PidTagTextAttachmentCharset,
    PidTagThumbnailPhoto,
    PidTagTimeZone,
    PidTagTitle,
    PidTagTnefCorrelationKey,
    PidTagToDoItemFlags,
    PidTagTransmittableDisplayName,
    PidTagTransportMessageHeaders,
    PidTagTransportMessageHeadersW,
    PidTagTrustSender,
    PidTagTtyTddPhoneNumber,
    PidTagUrlCompName,
    PidTagUrlCompNameSet,
    PidTagUrlName,
    PidTagUserCertificate,
    PidTagUserEntryId,
    PidTagUserX509Certificate,
    PidTagValidFolderMask,
    PidTagViewDescriptorBinary,
    PidTagViewDescriptorFlags,
    PidTagViewDescriptorLinkTo,
    PidTagViewDescriptorName,
    PidTagViewDescriptorStrings,
    PidTagViewDescriptorVersion,
    PidTagViewDescriptorViewFolder,
    PidTagViewsEntryId,
    PidTagVoiceMessageAttachmentOrder,
    PidTagVoiceMessageDuration,
    PidTagVoiceMessageSenderName,
    PidTagWeddingAnniversary,
    PidTagWeekInterval,
    PidTagWlinkAddressBookEID,
    PidTagWlinkAddressBookStoreEID,
    PidTagWlinkCalendarColor,
    PidTagWlinkClientID,
    PidTagWlinkEntryId,
    PidTagWlinkFlags,
    PidTagWlinkFolderType,
    PidTagWlinkGroupClsid,
    PidTagWlinkGroupHeaderID,
    PidTagWlinkGroupName,
    PidTagWlinkOrdinal,
    PidTagWlinkRecordKey,
    PidTagWlinkROGroupType,
    PidTagWlinkSaveStamp,
    PidTagWlinkSection,
    PidTagWlinkStoreEntryId,
    PidTagWlinkType,
    PidTagYearInterval,

    // Named Properties now
    StreamGuid,
    StreamEntry,
    StreamString,
    StreamEntries,
    Unknown,
}