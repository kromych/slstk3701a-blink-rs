#[doc = "Register `CH1CTRL` reader"]
pub type R = crate::R<CH1CTRL_SPEC>;
#[doc = "Register `CH1CTRL` writer"]
pub type W = crate::W<CH1CTRL_SPEC>;
#[doc = "Field `CONVMODE` reader - Conversion Mode"]
pub type CONVMODE_R = crate::BitReader;
#[doc = "Field `CONVMODE` writer - Conversion Mode"]
pub type CONVMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIGMODE` reader - Channel 1 Trigger Mode"]
pub type TRIGMODE_R = crate::FieldReader<TRIGMODE_A>;
#[doc = "Channel 1 Trigger Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRIGMODE_A {
    #[doc = "0: Channel 1 is triggered by CH1DATA or COMBDATA write"]
    SW = 0,
    #[doc = "1: Channel 1 is triggered by PRS input"]
    PRS = 1,
    #[doc = "2: Channel 1 is triggered by Refresh timer"]
    REFRESH = 2,
    #[doc = "3: Channel 1 is triggered by CH1DATA/COMBDATA write or PRS input"]
    SWPRS = 3,
    #[doc = "4: Channel 1 is triggered by CH1DATA/COMBDATA write or Refresh timer"]
    SWREFRESH = 4,
    #[doc = "5: Channel 1 is triggered by LESENSE"]
    LESENSE = 5,
}
impl From<TRIGMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TRIGMODE_A {
    type Ux = u8;
}
impl TRIGMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TRIGMODE_A> {
        match self.bits {
            0 => Some(TRIGMODE_A::SW),
            1 => Some(TRIGMODE_A::PRS),
            2 => Some(TRIGMODE_A::REFRESH),
            3 => Some(TRIGMODE_A::SWPRS),
            4 => Some(TRIGMODE_A::SWREFRESH),
            5 => Some(TRIGMODE_A::LESENSE),
            _ => None,
        }
    }
    #[doc = "Channel 1 is triggered by CH1DATA or COMBDATA write"]
    #[inline(always)]
    pub fn is_sw(&self) -> bool {
        *self == TRIGMODE_A::SW
    }
    #[doc = "Channel 1 is triggered by PRS input"]
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == TRIGMODE_A::PRS
    }
    #[doc = "Channel 1 is triggered by Refresh timer"]
    #[inline(always)]
    pub fn is_refresh(&self) -> bool {
        *self == TRIGMODE_A::REFRESH
    }
    #[doc = "Channel 1 is triggered by CH1DATA/COMBDATA write or PRS input"]
    #[inline(always)]
    pub fn is_swprs(&self) -> bool {
        *self == TRIGMODE_A::SWPRS
    }
    #[doc = "Channel 1 is triggered by CH1DATA/COMBDATA write or Refresh timer"]
    #[inline(always)]
    pub fn is_swrefresh(&self) -> bool {
        *self == TRIGMODE_A::SWREFRESH
    }
    #[doc = "Channel 1 is triggered by LESENSE"]
    #[inline(always)]
    pub fn is_lesense(&self) -> bool {
        *self == TRIGMODE_A::LESENSE
    }
}
#[doc = "Field `TRIGMODE` writer - Channel 1 Trigger Mode"]
pub type TRIGMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TRIGMODE_A>;
impl<'a, REG> TRIGMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel 1 is triggered by CH1DATA or COMBDATA write"]
    #[inline(always)]
    pub fn sw(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGMODE_A::SW)
    }
    #[doc = "Channel 1 is triggered by PRS input"]
    #[inline(always)]
    pub fn prs(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGMODE_A::PRS)
    }
    #[doc = "Channel 1 is triggered by Refresh timer"]
    #[inline(always)]
    pub fn refresh(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGMODE_A::REFRESH)
    }
    #[doc = "Channel 1 is triggered by CH1DATA/COMBDATA write or PRS input"]
    #[inline(always)]
    pub fn swprs(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGMODE_A::SWPRS)
    }
    #[doc = "Channel 1 is triggered by CH1DATA/COMBDATA write or Refresh timer"]
    #[inline(always)]
    pub fn swrefresh(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGMODE_A::SWREFRESH)
    }
    #[doc = "Channel 1 is triggered by LESENSE"]
    #[inline(always)]
    pub fn lesense(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGMODE_A::LESENSE)
    }
}
#[doc = "Field `PRSASYNC` reader - Channel 1 PRS Asynchronous Enable"]
pub type PRSASYNC_R = crate::BitReader;
#[doc = "Field `PRSASYNC` writer - Channel 1 PRS Asynchronous Enable"]
pub type PRSASYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRSSEL` reader - Channel 1 PRS Trigger Select"]
pub type PRSSEL_R = crate::FieldReader<PRSSEL_A>;
#[doc = "Channel 1 PRS Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSEL_A {
    #[doc = "0: PRS ch 0 triggers a conversion."]
    PRSCH0 = 0,
    #[doc = "1: PRS ch 1 triggers a conversion."]
    PRSCH1 = 1,
    #[doc = "2: PRS ch 2 triggers a conversion."]
    PRSCH2 = 2,
    #[doc = "3: PRS ch 3 triggers a conversion."]
    PRSCH3 = 3,
    #[doc = "4: PRS ch 4 triggers a conversion."]
    PRSCH4 = 4,
    #[doc = "5: PRS ch 5 triggers a conversion."]
    PRSCH5 = 5,
    #[doc = "6: PRS ch 6 triggers a conversion."]
    PRSCH6 = 6,
    #[doc = "7: PRS ch 7 triggers a conversion."]
    PRSCH7 = 7,
    #[doc = "8: PRS ch 8 triggers a conversion."]
    PRSCH8 = 8,
    #[doc = "9: PRS ch 9 triggers a conversion."]
    PRSCH9 = 9,
    #[doc = "10: PRS ch 10 triggers a conversion."]
    PRSCH10 = 10,
    #[doc = "11: PRS ch 11 triggers a conversion."]
    PRSCH11 = 11,
    #[doc = "12: PRS ch 12 triggers a conversion."]
    PRSCH12 = 12,
    #[doc = "13: PRS ch 13 triggers a conversion."]
    PRSCH13 = 13,
    #[doc = "14: PRS ch 14 triggers a conversion."]
    PRSCH14 = 14,
    #[doc = "15: PRS ch 15 triggers a conversion."]
    PRSCH15 = 15,
    #[doc = "16: PRS ch 16 triggers a conversion."]
    PRSCH16 = 16,
    #[doc = "17: PRS ch 17 triggers a conversion."]
    PRSCH17 = 17,
    #[doc = "18: PRS ch 18 triggers a conversion."]
    PRSCH18 = 18,
    #[doc = "19: PRS ch 19 triggers a conversion."]
    PRSCH19 = 19,
    #[doc = "20: PRS ch 20 triggers a conversion."]
    PRSCH20 = 20,
    #[doc = "21: PRS ch 21 triggers a conversion."]
    PRSCH21 = 21,
    #[doc = "22: PRS ch 22 triggers a conversion."]
    PRSCH22 = 22,
    #[doc = "23: PRS ch 23 triggers a conversion."]
    PRSCH23 = 23,
}
impl From<PRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRSSEL_A {
    type Ux = u8;
}
impl PRSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRSSEL_A> {
        match self.bits {
            0 => Some(PRSSEL_A::PRSCH0),
            1 => Some(PRSSEL_A::PRSCH1),
            2 => Some(PRSSEL_A::PRSCH2),
            3 => Some(PRSSEL_A::PRSCH3),
            4 => Some(PRSSEL_A::PRSCH4),
            5 => Some(PRSSEL_A::PRSCH5),
            6 => Some(PRSSEL_A::PRSCH6),
            7 => Some(PRSSEL_A::PRSCH7),
            8 => Some(PRSSEL_A::PRSCH8),
            9 => Some(PRSSEL_A::PRSCH9),
            10 => Some(PRSSEL_A::PRSCH10),
            11 => Some(PRSSEL_A::PRSCH11),
            12 => Some(PRSSEL_A::PRSCH12),
            13 => Some(PRSSEL_A::PRSCH13),
            14 => Some(PRSSEL_A::PRSCH14),
            15 => Some(PRSSEL_A::PRSCH15),
            16 => Some(PRSSEL_A::PRSCH16),
            17 => Some(PRSSEL_A::PRSCH17),
            18 => Some(PRSSEL_A::PRSCH18),
            19 => Some(PRSSEL_A::PRSCH19),
            20 => Some(PRSSEL_A::PRSCH20),
            21 => Some(PRSSEL_A::PRSCH21),
            22 => Some(PRSSEL_A::PRSCH22),
            23 => Some(PRSSEL_A::PRSCH23),
            _ => None,
        }
    }
    #[doc = "PRS ch 0 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL_A::PRSCH0
    }
    #[doc = "PRS ch 1 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL_A::PRSCH1
    }
    #[doc = "PRS ch 2 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL_A::PRSCH2
    }
    #[doc = "PRS ch 3 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL_A::PRSCH3
    }
    #[doc = "PRS ch 4 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL_A::PRSCH4
    }
    #[doc = "PRS ch 5 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL_A::PRSCH5
    }
    #[doc = "PRS ch 6 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSEL_A::PRSCH6
    }
    #[doc = "PRS ch 7 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSEL_A::PRSCH7
    }
    #[doc = "PRS ch 8 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSEL_A::PRSCH8
    }
    #[doc = "PRS ch 9 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSEL_A::PRSCH9
    }
    #[doc = "PRS ch 10 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSEL_A::PRSCH10
    }
    #[doc = "PRS ch 11 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSEL_A::PRSCH11
    }
    #[doc = "PRS ch 12 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == PRSSEL_A::PRSCH12
    }
    #[doc = "PRS ch 13 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == PRSSEL_A::PRSCH13
    }
    #[doc = "PRS ch 14 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == PRSSEL_A::PRSCH14
    }
    #[doc = "PRS ch 15 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == PRSSEL_A::PRSCH15
    }
    #[doc = "PRS ch 16 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == PRSSEL_A::PRSCH16
    }
    #[doc = "PRS ch 17 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == PRSSEL_A::PRSCH17
    }
    #[doc = "PRS ch 18 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == PRSSEL_A::PRSCH18
    }
    #[doc = "PRS ch 19 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == PRSSEL_A::PRSCH19
    }
    #[doc = "PRS ch 20 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == PRSSEL_A::PRSCH20
    }
    #[doc = "PRS ch 21 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == PRSSEL_A::PRSCH21
    }
    #[doc = "PRS ch 22 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == PRSSEL_A::PRSCH22
    }
    #[doc = "PRS ch 23 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == PRSSEL_A::PRSCH23
    }
}
#[doc = "Field `PRSSEL` writer - Channel 1 PRS Trigger Select"]
pub type PRSSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5, PRSSEL_A>;
impl<'a, REG> PRSSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS ch 0 triggers a conversion."]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH0)
    }
    #[doc = "PRS ch 1 triggers a conversion."]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH1)
    }
    #[doc = "PRS ch 2 triggers a conversion."]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH2)
    }
    #[doc = "PRS ch 3 triggers a conversion."]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH3)
    }
    #[doc = "PRS ch 4 triggers a conversion."]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH4)
    }
    #[doc = "PRS ch 5 triggers a conversion."]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH5)
    }
    #[doc = "PRS ch 6 triggers a conversion."]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH6)
    }
    #[doc = "PRS ch 7 triggers a conversion."]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH7)
    }
    #[doc = "PRS ch 8 triggers a conversion."]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH8)
    }
    #[doc = "PRS ch 9 triggers a conversion."]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH9)
    }
    #[doc = "PRS ch 10 triggers a conversion."]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH10)
    }
    #[doc = "PRS ch 11 triggers a conversion."]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH11)
    }
    #[doc = "PRS ch 12 triggers a conversion."]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH12)
    }
    #[doc = "PRS ch 13 triggers a conversion."]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH13)
    }
    #[doc = "PRS ch 14 triggers a conversion."]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH14)
    }
    #[doc = "PRS ch 15 triggers a conversion."]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH15)
    }
    #[doc = "PRS ch 16 triggers a conversion."]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH16)
    }
    #[doc = "PRS ch 17 triggers a conversion."]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH17)
    }
    #[doc = "PRS ch 18 triggers a conversion."]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH18)
    }
    #[doc = "PRS ch 19 triggers a conversion."]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH19)
    }
    #[doc = "PRS ch 20 triggers a conversion."]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH20)
    }
    #[doc = "PRS ch 21 triggers a conversion."]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH21)
    }
    #[doc = "PRS ch 22 triggers a conversion."]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH22)
    }
    #[doc = "PRS ch 23 triggers a conversion."]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH23)
    }
}
impl R {
    #[doc = "Bit 0 - Conversion Mode"]
    #[inline(always)]
    pub fn convmode(&self) -> CONVMODE_R {
        CONVMODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - Channel 1 Trigger Mode"]
    #[inline(always)]
    pub fn trigmode(&self) -> TRIGMODE_R {
        TRIGMODE_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Channel 1 PRS Asynchronous Enable"]
    #[inline(always)]
    pub fn prsasync(&self) -> PRSASYNC_R {
        PRSASYNC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:16 - Channel 1 PRS Trigger Select"]
    #[inline(always)]
    pub fn prssel(&self) -> PRSSEL_R {
        PRSSEL_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Conversion Mode"]
    #[inline(always)]
    #[must_use]
    pub fn convmode(&mut self) -> CONVMODE_W<CH1CTRL_SPEC> {
        CONVMODE_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Channel 1 Trigger Mode"]
    #[inline(always)]
    #[must_use]
    pub fn trigmode(&mut self) -> TRIGMODE_W<CH1CTRL_SPEC> {
        TRIGMODE_W::new(self, 4)
    }
    #[doc = "Bit 8 - Channel 1 PRS Asynchronous Enable"]
    #[inline(always)]
    #[must_use]
    pub fn prsasync(&mut self) -> PRSASYNC_W<CH1CTRL_SPEC> {
        PRSASYNC_W::new(self, 8)
    }
    #[doc = "Bits 12:16 - Channel 1 PRS Trigger Select"]
    #[inline(always)]
    #[must_use]
    pub fn prssel(&mut self) -> PRSSEL_W<CH1CTRL_SPEC> {
        PRSSEL_W::new(self, 12)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel 1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH1CTRL_SPEC;
impl crate::RegisterSpec for CH1CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch1ctrl::R`](R) reader structure"]
impl crate::Readable for CH1CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch1ctrl::W`](W) writer structure"]
impl crate::Writable for CH1CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH1CTRL to value 0"]
impl crate::Resettable for CH1CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
