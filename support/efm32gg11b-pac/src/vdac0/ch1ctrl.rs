#[doc = "Register `CH1CTRL` reader"]
pub type R = crate::R<Ch1ctrlSpec>;
#[doc = "Register `CH1CTRL` writer"]
pub type W = crate::W<Ch1ctrlSpec>;
#[doc = "Field `CONVMODE` reader - Conversion Mode"]
pub type ConvmodeR = crate::BitReader;
#[doc = "Field `CONVMODE` writer - Conversion Mode"]
pub type ConvmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Channel 1 Trigger Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trigmode {
    #[doc = "0: Channel 1 is triggered by CH1DATA or COMBDATA write"]
    Sw = 0,
    #[doc = "1: Channel 1 is triggered by PRS input"]
    Prs = 1,
    #[doc = "2: Channel 1 is triggered by Refresh timer"]
    Refresh = 2,
    #[doc = "3: Channel 1 is triggered by CH1DATA/COMBDATA write or PRS input"]
    Swprs = 3,
    #[doc = "4: Channel 1 is triggered by CH1DATA/COMBDATA write or Refresh timer"]
    Swrefresh = 4,
    #[doc = "5: Channel 1 is triggered by LESENSE"]
    Lesense = 5,
}
impl From<Trigmode> for u8 {
    #[inline(always)]
    fn from(variant: Trigmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trigmode {
    type Ux = u8;
}
impl crate::IsEnum for Trigmode {}
#[doc = "Field `TRIGMODE` reader - Channel 1 Trigger Mode"]
pub type TrigmodeR = crate::FieldReader<Trigmode>;
impl TrigmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Trigmode> {
        match self.bits {
            0 => Some(Trigmode::Sw),
            1 => Some(Trigmode::Prs),
            2 => Some(Trigmode::Refresh),
            3 => Some(Trigmode::Swprs),
            4 => Some(Trigmode::Swrefresh),
            5 => Some(Trigmode::Lesense),
            _ => None,
        }
    }
    #[doc = "Channel 1 is triggered by CH1DATA or COMBDATA write"]
    #[inline(always)]
    pub fn is_sw(&self) -> bool {
        *self == Trigmode::Sw
    }
    #[doc = "Channel 1 is triggered by PRS input"]
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == Trigmode::Prs
    }
    #[doc = "Channel 1 is triggered by Refresh timer"]
    #[inline(always)]
    pub fn is_refresh(&self) -> bool {
        *self == Trigmode::Refresh
    }
    #[doc = "Channel 1 is triggered by CH1DATA/COMBDATA write or PRS input"]
    #[inline(always)]
    pub fn is_swprs(&self) -> bool {
        *self == Trigmode::Swprs
    }
    #[doc = "Channel 1 is triggered by CH1DATA/COMBDATA write or Refresh timer"]
    #[inline(always)]
    pub fn is_swrefresh(&self) -> bool {
        *self == Trigmode::Swrefresh
    }
    #[doc = "Channel 1 is triggered by LESENSE"]
    #[inline(always)]
    pub fn is_lesense(&self) -> bool {
        *self == Trigmode::Lesense
    }
}
#[doc = "Field `TRIGMODE` writer - Channel 1 Trigger Mode"]
pub type TrigmodeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Trigmode>;
impl<'a, REG> TrigmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel 1 is triggered by CH1DATA or COMBDATA write"]
    #[inline(always)]
    pub fn sw(self) -> &'a mut crate::W<REG> {
        self.variant(Trigmode::Sw)
    }
    #[doc = "Channel 1 is triggered by PRS input"]
    #[inline(always)]
    pub fn prs(self) -> &'a mut crate::W<REG> {
        self.variant(Trigmode::Prs)
    }
    #[doc = "Channel 1 is triggered by Refresh timer"]
    #[inline(always)]
    pub fn refresh(self) -> &'a mut crate::W<REG> {
        self.variant(Trigmode::Refresh)
    }
    #[doc = "Channel 1 is triggered by CH1DATA/COMBDATA write or PRS input"]
    #[inline(always)]
    pub fn swprs(self) -> &'a mut crate::W<REG> {
        self.variant(Trigmode::Swprs)
    }
    #[doc = "Channel 1 is triggered by CH1DATA/COMBDATA write or Refresh timer"]
    #[inline(always)]
    pub fn swrefresh(self) -> &'a mut crate::W<REG> {
        self.variant(Trigmode::Swrefresh)
    }
    #[doc = "Channel 1 is triggered by LESENSE"]
    #[inline(always)]
    pub fn lesense(self) -> &'a mut crate::W<REG> {
        self.variant(Trigmode::Lesense)
    }
}
#[doc = "Field `PRSASYNC` reader - Channel 1 PRS Asynchronous Enable"]
pub type PrsasyncR = crate::BitReader;
#[doc = "Field `PRSASYNC` writer - Channel 1 PRS Asynchronous Enable"]
pub type PrsasyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Channel 1 PRS Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prssel {
    #[doc = "0: PRS ch 0 triggers a conversion."]
    Prsch0 = 0,
    #[doc = "1: PRS ch 1 triggers a conversion."]
    Prsch1 = 1,
    #[doc = "2: PRS ch 2 triggers a conversion."]
    Prsch2 = 2,
    #[doc = "3: PRS ch 3 triggers a conversion."]
    Prsch3 = 3,
    #[doc = "4: PRS ch 4 triggers a conversion."]
    Prsch4 = 4,
    #[doc = "5: PRS ch 5 triggers a conversion."]
    Prsch5 = 5,
    #[doc = "6: PRS ch 6 triggers a conversion."]
    Prsch6 = 6,
    #[doc = "7: PRS ch 7 triggers a conversion."]
    Prsch7 = 7,
    #[doc = "8: PRS ch 8 triggers a conversion."]
    Prsch8 = 8,
    #[doc = "9: PRS ch 9 triggers a conversion."]
    Prsch9 = 9,
    #[doc = "10: PRS ch 10 triggers a conversion."]
    Prsch10 = 10,
    #[doc = "11: PRS ch 11 triggers a conversion."]
    Prsch11 = 11,
    #[doc = "12: PRS ch 12 triggers a conversion."]
    Prsch12 = 12,
    #[doc = "13: PRS ch 13 triggers a conversion."]
    Prsch13 = 13,
    #[doc = "14: PRS ch 14 triggers a conversion."]
    Prsch14 = 14,
    #[doc = "15: PRS ch 15 triggers a conversion."]
    Prsch15 = 15,
    #[doc = "16: PRS ch 16 triggers a conversion."]
    Prsch16 = 16,
    #[doc = "17: PRS ch 17 triggers a conversion."]
    Prsch17 = 17,
    #[doc = "18: PRS ch 18 triggers a conversion."]
    Prsch18 = 18,
    #[doc = "19: PRS ch 19 triggers a conversion."]
    Prsch19 = 19,
    #[doc = "20: PRS ch 20 triggers a conversion."]
    Prsch20 = 20,
    #[doc = "21: PRS ch 21 triggers a conversion."]
    Prsch21 = 21,
    #[doc = "22: PRS ch 22 triggers a conversion."]
    Prsch22 = 22,
    #[doc = "23: PRS ch 23 triggers a conversion."]
    Prsch23 = 23,
}
impl From<Prssel> for u8 {
    #[inline(always)]
    fn from(variant: Prssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prssel {
    type Ux = u8;
}
impl crate::IsEnum for Prssel {}
#[doc = "Field `PRSSEL` reader - Channel 1 PRS Trigger Select"]
pub type PrsselR = crate::FieldReader<Prssel>;
impl PrsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prssel> {
        match self.bits {
            0 => Some(Prssel::Prsch0),
            1 => Some(Prssel::Prsch1),
            2 => Some(Prssel::Prsch2),
            3 => Some(Prssel::Prsch3),
            4 => Some(Prssel::Prsch4),
            5 => Some(Prssel::Prsch5),
            6 => Some(Prssel::Prsch6),
            7 => Some(Prssel::Prsch7),
            8 => Some(Prssel::Prsch8),
            9 => Some(Prssel::Prsch9),
            10 => Some(Prssel::Prsch10),
            11 => Some(Prssel::Prsch11),
            12 => Some(Prssel::Prsch12),
            13 => Some(Prssel::Prsch13),
            14 => Some(Prssel::Prsch14),
            15 => Some(Prssel::Prsch15),
            16 => Some(Prssel::Prsch16),
            17 => Some(Prssel::Prsch17),
            18 => Some(Prssel::Prsch18),
            19 => Some(Prssel::Prsch19),
            20 => Some(Prssel::Prsch20),
            21 => Some(Prssel::Prsch21),
            22 => Some(Prssel::Prsch22),
            23 => Some(Prssel::Prsch23),
            _ => None,
        }
    }
    #[doc = "PRS ch 0 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == Prssel::Prsch0
    }
    #[doc = "PRS ch 1 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == Prssel::Prsch1
    }
    #[doc = "PRS ch 2 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == Prssel::Prsch2
    }
    #[doc = "PRS ch 3 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == Prssel::Prsch3
    }
    #[doc = "PRS ch 4 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == Prssel::Prsch4
    }
    #[doc = "PRS ch 5 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == Prssel::Prsch5
    }
    #[doc = "PRS ch 6 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == Prssel::Prsch6
    }
    #[doc = "PRS ch 7 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == Prssel::Prsch7
    }
    #[doc = "PRS ch 8 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == Prssel::Prsch8
    }
    #[doc = "PRS ch 9 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == Prssel::Prsch9
    }
    #[doc = "PRS ch 10 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == Prssel::Prsch10
    }
    #[doc = "PRS ch 11 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == Prssel::Prsch11
    }
    #[doc = "PRS ch 12 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == Prssel::Prsch12
    }
    #[doc = "PRS ch 13 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == Prssel::Prsch13
    }
    #[doc = "PRS ch 14 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == Prssel::Prsch14
    }
    #[doc = "PRS ch 15 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == Prssel::Prsch15
    }
    #[doc = "PRS ch 16 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == Prssel::Prsch16
    }
    #[doc = "PRS ch 17 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == Prssel::Prsch17
    }
    #[doc = "PRS ch 18 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == Prssel::Prsch18
    }
    #[doc = "PRS ch 19 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == Prssel::Prsch19
    }
    #[doc = "PRS ch 20 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == Prssel::Prsch20
    }
    #[doc = "PRS ch 21 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == Prssel::Prsch21
    }
    #[doc = "PRS ch 22 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == Prssel::Prsch22
    }
    #[doc = "PRS ch 23 triggers a conversion."]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == Prssel::Prsch23
    }
}
#[doc = "Field `PRSSEL` writer - Channel 1 PRS Trigger Select"]
pub type PrsselW<'a, REG> = crate::FieldWriter<'a, REG, 5, Prssel>;
impl<'a, REG> PrsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS ch 0 triggers a conversion."]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch0)
    }
    #[doc = "PRS ch 1 triggers a conversion."]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch1)
    }
    #[doc = "PRS ch 2 triggers a conversion."]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch2)
    }
    #[doc = "PRS ch 3 triggers a conversion."]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch3)
    }
    #[doc = "PRS ch 4 triggers a conversion."]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch4)
    }
    #[doc = "PRS ch 5 triggers a conversion."]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch5)
    }
    #[doc = "PRS ch 6 triggers a conversion."]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch6)
    }
    #[doc = "PRS ch 7 triggers a conversion."]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch7)
    }
    #[doc = "PRS ch 8 triggers a conversion."]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch8)
    }
    #[doc = "PRS ch 9 triggers a conversion."]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch9)
    }
    #[doc = "PRS ch 10 triggers a conversion."]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch10)
    }
    #[doc = "PRS ch 11 triggers a conversion."]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch11)
    }
    #[doc = "PRS ch 12 triggers a conversion."]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch12)
    }
    #[doc = "PRS ch 13 triggers a conversion."]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch13)
    }
    #[doc = "PRS ch 14 triggers a conversion."]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch14)
    }
    #[doc = "PRS ch 15 triggers a conversion."]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch15)
    }
    #[doc = "PRS ch 16 triggers a conversion."]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch16)
    }
    #[doc = "PRS ch 17 triggers a conversion."]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch17)
    }
    #[doc = "PRS ch 18 triggers a conversion."]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch18)
    }
    #[doc = "PRS ch 19 triggers a conversion."]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch19)
    }
    #[doc = "PRS ch 20 triggers a conversion."]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch20)
    }
    #[doc = "PRS ch 21 triggers a conversion."]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch21)
    }
    #[doc = "PRS ch 22 triggers a conversion."]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch22)
    }
    #[doc = "PRS ch 23 triggers a conversion."]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch23)
    }
}
impl R {
    #[doc = "Bit 0 - Conversion Mode"]
    #[inline(always)]
    pub fn convmode(&self) -> ConvmodeR {
        ConvmodeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - Channel 1 Trigger Mode"]
    #[inline(always)]
    pub fn trigmode(&self) -> TrigmodeR {
        TrigmodeR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Channel 1 PRS Asynchronous Enable"]
    #[inline(always)]
    pub fn prsasync(&self) -> PrsasyncR {
        PrsasyncR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:16 - Channel 1 PRS Trigger Select"]
    #[inline(always)]
    pub fn prssel(&self) -> PrsselR {
        PrsselR::new(((self.bits >> 12) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Conversion Mode"]
    #[inline(always)]
    pub fn convmode(&mut self) -> ConvmodeW<'_, Ch1ctrlSpec> {
        ConvmodeW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Channel 1 Trigger Mode"]
    #[inline(always)]
    pub fn trigmode(&mut self) -> TrigmodeW<'_, Ch1ctrlSpec> {
        TrigmodeW::new(self, 4)
    }
    #[doc = "Bit 8 - Channel 1 PRS Asynchronous Enable"]
    #[inline(always)]
    pub fn prsasync(&mut self) -> PrsasyncW<'_, Ch1ctrlSpec> {
        PrsasyncW::new(self, 8)
    }
    #[doc = "Bits 12:16 - Channel 1 PRS Trigger Select"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PrsselW<'_, Ch1ctrlSpec> {
        PrsselW::new(self, 12)
    }
}
#[doc = "Channel 1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch1ctrlSpec;
impl crate::RegisterSpec for Ch1ctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch1ctrl::R`](R) reader structure"]
impl crate::Readable for Ch1ctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ch1ctrl::W`](W) writer structure"]
impl crate::Writable for Ch1ctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH1CTRL to value 0"]
impl crate::Resettable for Ch1ctrlSpec {}
